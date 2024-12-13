use crate::{
    inst::{Inst, RunError, RunInst},
    value::Value,
};

#[derive(Debug)]
pub struct VM {
    stack: Vec<Value>,
}
impl VM {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn execute(&mut self, inst: impl Into<Inst>) -> Result<(), RunError> {
        inst.into().run(self)
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
}

#[derive(Debug)]
pub enum VMError {
    StackUnderflow,
    StackReadOutOfBound,
    StackWriteOutOfBound,
}
