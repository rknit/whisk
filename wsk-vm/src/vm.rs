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
}

#[derive(Debug)]
pub enum VMError {
    StackUnderflow,
}
