use inst::{Cmp, Inst, RunError};
use vm::VM;

pub mod inst;
pub mod value;
pub mod vm;

fn main() -> Result<(), RunError> {
    let mut vm = VM::new();

    vm.execute(Inst::Push(9.into()))?;
    vm.execute(Inst::Push(9.into()))?;
    vm.execute(Cmp::Greater)?;

    dbg!(&vm);

    Ok(())
}
