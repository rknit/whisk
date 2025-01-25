use std::ops::DerefMut;

use crate::Value;

use super::abi::Register;

const VM_REG_COUNT: usize = 32;
const VM_STACK_LEN: usize = 8192;

#[derive(Debug, Clone)]
pub struct VM {
    regs: Box<[Value; VM_REG_COUNT]>,
    stack: Box<[Value; VM_STACK_LEN]>,
    frames: Vec<Frame>,
    halted: bool,
}
impl Default for VM {
    fn default() -> Self {
        Self {
            regs: Box::new([Value::Int(0); VM_REG_COUNT]),
            stack: Box::new([Value::Int(0); VM_STACK_LEN]),
            frames: vec![Frame::default()],
            halted: false,
        }
    }
}
impl VM {
    pub fn execute(&mut self) -> Result<(), VMError> {
        self.reset();

        while !self.is_halted() {}

        Ok(())
    }

    pub fn reset(&mut self) {
        self.frames.resize_with(1, Frame::default);
        self.halted = false;

        for v in self.regs.deref_mut() {
            *v = Value::Int(0);
        }
    }

    pub fn push_value(&mut self, v: Value) -> Result<(), VMError> {
        self.stack[self.get_frame().state.sp] = v;
        self.get_frame_mut().inc_stack_pointer()
    }

    pub fn pop_value(&mut self) -> Result<Value, VMError> {
        let frame = self.get_frame_mut();
        frame.dec_stack_pointer()?;
        Ok(self.stack[frame.state.sp])
    }

    pub fn push_frame(&mut self) {
        self.frames.push(Frame::default());
    }

    pub fn pop_frame(&mut self) -> Result<(), VMError> {
        self.frames
            .pop()
            .map(|_| ())
            .ok_or(VMError::StackFrameUnderflow)
    }

    pub fn get_frame(&self) -> &Frame {
        self.frames.last().unwrap()
    }

    pub fn get_frame_mut(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    pub fn get_reg(&self, r: Register) -> &Value {
        &self.regs[r.get_index()]
    }

    pub fn get_reg_mut(&mut self, r: Register) -> &mut Value {
        &mut self.regs[r.get_index()]
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
    StackFrameUnderflow,
    StackUnderflow,
    StackOverflow,
}

#[derive(Debug, Default, Clone, Copy)]
struct VMState {
    pub pc: usize,
    pub sp: usize,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Frame {
    state: VMState,
}
impl Frame {
    pub fn advance(&mut self) {
        self.state.pc = self.state.pc.wrapping_add(1);
    }

    pub fn inc_stack_pointer(&mut self) -> Result<(), VMError> {
        self.state.sp = self.state.pc.checked_add(1).ok_or(VMError::StackOverflow)?;
        Ok(())
    }

    pub fn dec_stack_pointer(&mut self) -> Result<(), VMError> {
        self.state.sp = self
            .state
            .pc
            .checked_sub(1)
            .ok_or(VMError::StackUnderflow)?;
        Ok(())
    }

    pub fn jump(&mut self, offset: isize) {
        self.state.pc = self.state.pc.wrapping_add_signed(offset);
    }
}
