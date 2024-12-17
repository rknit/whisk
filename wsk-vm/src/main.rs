use wsk_vm::{Cmp, VM};
use wsk_vm::{Inst, RunError};

fn main() -> Result<(), RunError> {
    let mut vm = VM::new();

    let program = vec![
        Inst::Push(0.into()),
        Inst::Push(1.into()),
        Inst::Add,
        Inst::Push(10.into()),
        Cmp::Less.into(),
        Inst::JmpTrue(-4),
        Inst::Halt,
    ];
    vm.execute(program).map_err(|e| {
        eprintln!("{:#?}", vm);
        e
    })?;

    dbg!(&vm);

    Ok(())
}
