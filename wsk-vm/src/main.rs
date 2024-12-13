use wsk_vm::VM;
use wsk_vm::{Inst, RunError};

fn main() -> Result<(), RunError> {
    let mut vm = VM::new();

    vm.execute(Inst::Push(9.into()))?;
    vm.execute(Inst::Push(7.into()))?;
    vm.execute(Inst::Push(2.into()))?;
    vm.execute(Inst::Push(111.into()))?;
    vm.execute(Inst::Load(0))?;

    dbg!(&vm);

    Ok(())
}
