use std::collections::HashMap;

use crate::{
    inst::{RunError, RunInst},
    program::Program,
    value::Value,
};

#[derive(Debug, Default)]
pub struct VM {
    stack: Vec<Value>,
    frames: Vec<Frame>,
    status: VMStatus,
}
impl VM {
    pub fn reset(&mut self, entry_point: usize) {
        self.stack.clear();
        self.frames.clear();
        self.push_frame(entry_point);
        self.status = VMStatus::default();
    }

    pub fn execute(&mut self, program: Program) -> Result<(), RunError> {
        self.reset(program.get_entry_point());

        while !self.is_halted() {
            let Frame { fi, pc, .. } = self.get_frame();
            let Some(func) = program.get(*fi) else {
                return Err(VMError::InvalidFunctionIndex.into());
            };
            let Some(inst) = func.get(*pc) else {
                return Err(VMError::InstReadOutOfBound.into());
            };

            inst.run(self)?;

            if self.is_skipped() {
                self.status.skip = false;
            } else {
                self.get_frame_mut().advance();
            }
        }

        Ok(())
    }

    pub fn push_frame(&mut self, fi: usize) {
        self.frames.push(Frame::new(fi));
    }

    pub fn pop_frame(&mut self) -> Result<(), VMError> {
        if self.frames.len() <= 1 {
            return Err(VMError::StackFrameUnderflow);
        }
        self.frames.pop();
        Ok(())
    }

    pub fn get_frame(&self) -> &Frame {
        self.frames.last().unwrap()
    }

    fn get_frame_mut(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Result<Value, VMError> {
        self.stack.pop().ok_or(VMError::StackUnderflow)
    }

    pub fn store(&mut self, key: usize, value: Value) {
        self.get_frame_mut().store(key, value);
    }

    pub fn load(&self, key: usize) -> Result<Value, VMError> {
        self.get_frame().load(key)
    }

    pub fn jump(&mut self, offset: isize) {
        self.get_frame_mut().jump(offset);
        self.skip();
    }

    pub fn call(&mut self, fi: usize) {
        self.push_frame(fi);
        self.skip();
    }

    pub fn ret(&mut self) -> Result<(), RunError> {
        self.pop_frame()?;
        Ok(())
    }

    pub fn halt(&mut self) {
        self.status.halt = true;
    }

    pub fn is_halted(&self) -> bool {
        self.status.halt
    }

    pub fn skip(&mut self) {
        self.status.skip = true;
    }

    pub fn is_skipped(&self) -> bool {
        self.status.skip
    }
}

#[derive(Debug)]
pub enum VMError {
    InvalidFunctionIndex,
    InstReadOutOfBound,
    StackFrameUnderflow,
    StackUnderflow,
    StackReadOutOfBound,
    StackWriteOutOfBound,
    InvalidLocalId,
}

#[derive(Debug, Default)]
pub struct VMStatus {
    halt: bool,
    skip: bool,
}

#[derive(Debug)]
pub struct Frame {
    fi: usize,
    pc: usize,
    locals: HashMap<usize, Value>,
}
impl Frame {
    pub fn new(fi: usize) -> Self {
        Self {
            fi,
            pc: 0,
            locals: HashMap::new(),
        }
    }

    pub fn store(&mut self, key: usize, value: Value) {
        if let Some(v) = self.locals.get_mut(&key) {
            *v = value;
        } else {
            self.locals.insert(key, value);
        }
    }

    pub fn load(&self, key: usize) -> Result<Value, VMError> {
        self.locals
            .get(&key)
            .copied()
            .ok_or(VMError::InvalidLocalId)
    }

    pub fn advance(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }

    pub fn jump(&mut self, offset: isize) {
        self.pc = self.pc.wrapping_add_signed(offset);
    }
}
