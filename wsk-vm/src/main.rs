use wsk_vm::program::{Function, Program};
use wsk_vm::{Cmp, VM};
use wsk_vm::{Inst, RunError};

fn main() -> Result<(), RunError> {
    let mut vm = VM::new();

    let mut func = Function::new();
    func.push_insts([
        Inst::Push(0.into()),
        Inst::Push(1.into()),
        Inst::Add,
        Inst::Push(10.into()),
        Cmp::Less.into(),
        Inst::JmpTrue(-4),
        Inst::Halt,
    ]);

    let mut program = Program::new();
    program.push_func(func);

    vm.execute(program).map_err(|e| {
        eprintln!("{:#?}", vm);
        e
    })?;

    dbg!(&vm);

    Ok(())
}
