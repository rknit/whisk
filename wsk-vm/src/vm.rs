use crate::{
    inst::{Inst, RunError, RunInst},
    value::Value,
};

#[derive(Debug)]
pub struct VM {
    stack: Vec<Value>,
    pc: usize,
    status: VMStatus,
}
impl VM {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            pc: 0,
            status: VMStatus::default(),
        }
    }

    pub fn execute(&mut self, program: Vec<Inst>) -> Result<(), RunError> {
        self.pc = 0;
        self.status.halt = false;
        while !self.is_halted() {
            let Some(inst) = program.get(self.pc) else {
                return Err(VMError::InstReadOutOfBound.into());
            };
            inst.run(self)?;
            self.pc = self.pc.wrapping_add(1);
        }
        Ok(())
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Result<Value, VMError> {
        self.stack.pop().ok_or(VMError::StackUnderflow)
    }

    /// Read the n-th value from the top of the stack.
    pub fn read_stack(&self, idx: usize) -> Result<&Value, VMError> {
        if idx >= self.stack.len() {
            Err(VMError::StackReadOutOfBound)
        } else {
            Ok(self.stack.get(self.stack.len() - 1 - idx).unwrap())
        }
    }

    /// Replace the n-th value from the top of the stack with the provided value.
    pub fn replace_stack(&mut self, idx: usize, value: Value) -> Result<(), VMError> {
        if idx >= self.stack.len() {
            Err(VMError::StackWriteOutOfBound)
        } else {
            let ridx = self.stack.len() - 1 - idx;
            *self.stack.get_mut(ridx).unwrap() = value;
            Ok(())
        }
    }

    pub fn halt(&mut self) {
        self.status.halt = true;
    }

    pub fn is_halted(&self) -> bool {
        self.status.halt
    }

    pub fn jump(&mut self, offset: isize) {
        self.pc = self.pc.wrapping_add_signed(offset);
        self.pc = self.pc.wrapping_sub(1);
    }
}

#[derive(Debug)]
pub enum VMError {
    InstReadOutOfBound,
    StackUnderflow,
    StackReadOutOfBound,
    StackWriteOutOfBound,
}

#[derive(Debug, Default)]
pub struct VMStatus {
    halt: bool,
}
