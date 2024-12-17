use crate::{
    inst::{RunError, RunInst},
    program::Program,
    value::Value,
};

#[derive(Debug)]
pub struct VM {
    stack: Vec<Value>,
    frames: Vec<(usize, usize)>,
    status: VMStatus,
}
impl VM {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            frames: vec![(0, 0)],
            status: VMStatus::default(),
        }
    }

    pub fn reset(&mut self, entry_point: usize) {
        self.stack.clear();
        self.frames.clear();
        self.push_frame(entry_point);
        self.status = VMStatus::default();
    }

    pub fn execute(&mut self, program: Program) -> Result<(), RunError> {
        self.reset(program.get_entry_point());

        while !self.is_halted() {
            let (fi, pc) = self.get_frame();
            let Some(func) = program.get(fi) else {
                return Err(VMError::InvalidFunctionIndex.into());
            };
            let Some(inst) = func.get(pc) else {
                return Err(VMError::InstReadOutOfBound.into());
            };

            inst.run(self)?;

            if self.is_skipped() {
                self.status.skip = false;
            } else {
                let (_, pc) = self.get_frame_mut();
                *pc = pc.wrapping_add(1);
            }
        }

        Ok(())
    }

    pub fn push_frame(&mut self, fi: usize) {
        self.frames.push((fi, 0));
    }

    pub fn pop_frame(&mut self) -> Result<(), VMError> {
        if self.frames.len() <= 1 {
            return Err(VMError::StackFrameUnderflow);
        }
        self.frames.pop();
        Ok(())
    }

    pub fn get_frame(&self) -> (usize, usize) {
        *self.frames.last().unwrap()
    }

    fn get_frame_mut(&mut self) -> &mut (usize, usize) {
        self.frames.last_mut().unwrap()
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
        let (_, pc) = self.get_frame_mut();
        *pc = pc.wrapping_add_signed(offset);
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
}

#[derive(Debug)]
pub enum VMError {
    InvalidFunctionIndex,
    InstReadOutOfBound,
    StackFrameUnderflow,
    StackUnderflow,
    StackReadOutOfBound,
    StackWriteOutOfBound,
}

#[derive(Debug, Default)]
pub struct VMStatus {
    halt: bool,
    skip: bool,
}
