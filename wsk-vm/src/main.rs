use wsk_vm::VM;
use wsk_vm::{Cmp, Inst, RunError};

fn main() -> Result<(), RunError> {
    let mut vm = VM::new();

    vm.execute(Inst::Push(9.into()))?;
    vm.execute(Inst::Push(9.into()))?;
    vm.execute(Cmp::Greater)?;

    dbg!(&vm);

    Ok(())
}
