use core::fmt;
use std::ops::DerefMut;

use crate::Value;

use super::{abi::Reg, inst::RunError, program::Program};

const VM_REG_COUNT: usize = 32;
const VM_STACK_LEN: usize = 8192;

#[derive(Clone)]
pub struct VM {
    regs: Box<[Value; VM_REG_COUNT]>,
    stack: Box<[Value; VM_STACK_LEN]>,
    frames: Vec<Frame>,
    status: Status,
    halted: bool,
}
impl Default for VM {
    fn default() -> Self {
        Self {
            regs: Box::new([Value::Int(0); VM_REG_COUNT]),
            stack: Box::new([Value::Int(0); VM_STACK_LEN]),
            frames: vec![Frame::default()],
            status: Status::default(),
            halted: false,
        }
    }
}
impl VM {
    pub fn execute(&mut self, prog: &Program) -> Result<(), RunError> {
        self.reset(Frame::new(prog.get_entry_point(), 0, 0));
        while !self.is_halted() {
            self.advance(prog)?;
        }
        Ok(())
    }

    pub fn advance(&mut self, prog: &Program) -> Result<(), RunError> {
        let func = prog
            .get(self.get_frame().get_fi())
            .ok_or(VMError::InvalidFunctionIndex)?;
        let inst = func
            .get(self.get_frame().get_pc())
            .ok_or(VMError::InstReadOutOfBound)?;
        inst.run(self)?;

        if self.status.skip {
            self.status.skip = false;
        } else {
            self.get_frame_mut().advance();
        }
        Ok(())
    }

    pub fn reset(&mut self, entry_frame: Frame) {
        self.frames = vec![entry_frame];
        self.status = Status::default();
        self.halted = false;

        for v in self.regs.deref_mut() {
            *v = Value::Int(0);
        }
    }

    pub fn call(&mut self, fi: usize) {
        self.push_frame(self.get_frame().new_call(fi));
        self.status.skip = true;
    }

    pub fn ret(&mut self) -> Result<(), VMError> {
        self.pop_frame()
    }

    pub fn jump(&mut self, offset: isize) {
        self.get_frame_mut().jump(offset);
        self.status.skip = true;
    }

    pub fn push_value(&mut self, v: Value) -> Result<(), VMError> {
        self.stack[self.get_frame().get_sp()] = v;
        self.get_frame_mut().inc_stack_pointer()?;
        Ok(())
    }

    pub fn pop_value(&mut self) -> Result<Value, VMError> {
        let frame = self.get_frame_mut();
        frame.dec_stack_pointer()?;
        Ok(self.stack[frame.get_sp()])
    }

    fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    fn pop_frame(&mut self) -> Result<(), VMError> {
        if self.frames.len() <= 1 {
            Err(VMError::StackFrameUnderflow)
        } else {
            self.frames.pop();
            Ok(())
        }
    }

    fn get_frame(&self) -> &Frame {
        self.frames.last().unwrap()
    }

    fn get_frame_mut(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    pub fn get_reg(&self, r: Reg) -> &Value {
        &self.regs[r.get_index() as usize]
    }

    pub fn get_reg_mut(&mut self, r: Reg) -> &mut Value {
        &mut self.regs[r.get_index() as usize]
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }
}

#[derive(Debug)]
pub enum VMError {
    InvalidFunctionIndex,
    InstReadOutOfBound,
    StackFrameUnderflow,
    StackUnderflow,
    StackOverflow,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Status {
    pub skip: bool,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Frame {
    fi: usize,
    pc: usize,
    sp: usize,
}
impl Frame {
    pub fn new(fi: usize, pc: usize, sp: usize) -> Self {
        Self { fi, pc, sp }
    }

    pub fn new_call(&self, fi: usize) -> Self {
        Self {
            fi,
            pc: 0,
            sp: self.sp,
        }
    }

    pub fn advance(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }

    pub fn inc_stack_pointer(&mut self) -> Result<(), VMError> {
        self.sp = self.sp.checked_add(1).ok_or(VMError::StackOverflow)?;
        Ok(())
    }

    pub fn dec_stack_pointer(&mut self) -> Result<(), VMError> {
        self.sp = self.sp.checked_sub(1).ok_or(VMError::StackUnderflow)?;
        Ok(())
    }

    pub fn jump(&mut self, offset: isize) {
        self.pc = self.pc.wrapping_add_signed(offset);
    }

    pub fn get_fi(&self) -> usize {
        self.fi
    }

    pub fn get_pc(&self) -> usize {
        self.pc
    }

    pub fn get_sp(&self) -> usize {
        self.sp
    }
}

impl fmt::Debug for VM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "VM:")?;
        writeln!(f, "\thalted: {:?}", self.halted)?;
        writeln!(f, "\tstatus: {:?}", self.status)?;
        writeln!(f, "\tregs:")?;
        for (i, reg) in self.regs.iter().enumerate() {
            writeln!(f, "\t\tr{}: {}", i, reg)?;
        }
        writeln!(f, "\tframes:")?;
        for (i, frame) in self.frames.iter().rev().enumerate() {
            writeln!(f, "\t\t#{}: {:?}", i, frame)?;
        }
        Ok(())
    }
}
