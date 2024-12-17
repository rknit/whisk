use crate::{
    inst::{RunError, RunInst},
    program::Program,
    value::Value,
};

#[derive(Debug)]
pub struct VM {
    stack: Vec<Value>,
    pc: usize,
    fi: usize,
    status: VMStatus,
}
impl VM {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            pc: 0,
            fi: 0,
            status: VMStatus::default(),
        }
    }

    pub fn execute(&mut self, program: Program) -> Result<(), RunError> {
        self.pc = 0;
        self.fi = program.get_entry_point();
        self.status = VMStatus::default();

        while !self.is_halted() {
            let Some(func) = program.get(self.fi) else {
                return Err(VMError::InvalidFunctionIndex.into());
            };
            let Some(inst) = func.get(self.pc) else {
                return Err(VMError::InstReadOutOfBound.into());
            };

            inst.run(self)?;

            if self.is_skipped() {
                self.status.skip = false;
            } else {
                self.pc = self.pc.wrapping_add(1);
            }
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

    pub fn skip(&mut self) {
        self.status.skip = true;
    }

    pub fn is_skipped(&self) -> bool {
        self.status.skip
    }

    pub fn jump(&mut self, offset: isize) {
        self.pc = self.pc.wrapping_add_signed(offset);
        self.skip();
    }
}

#[derive(Debug)]
pub enum VMError {
    InvalidFunctionIndex,
    InstReadOutOfBound,
    StackUnderflow,
    StackReadOutOfBound,
    StackWriteOutOfBound,
}

#[derive(Debug, Default)]
pub struct VMStatus {
    halt: bool,
    skip: bool,
}
