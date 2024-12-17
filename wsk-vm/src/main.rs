use wsk_vm::program::{Function, Program};
use wsk_vm::VM;
use wsk_vm::{Inst, RunError};

fn main() -> Result<(), RunError> {
    let mut vm = VM::new();
    let mut program = Program::new();

    let entry = program.add_func(Function::from_insts([
        Inst::Push(2.into()),
        Inst::Push(3.into()),
        Inst::Call(1),
        Inst::Halt,
    ]));
    program.set_entry_point(entry);

    program.add_func(Function::from_insts([Inst::Add, Inst::Ret]));

    vm.execute(program).map_err(|e| {
        eprintln!("{:#?}", vm);
        e
    })?;

    dbg!(&vm);

    Ok(())
}
