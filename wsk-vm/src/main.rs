use wsk_vm::program::{Function, Program};
use wsk_vm::VM;
use wsk_vm::{Inst, RunError};

fn main() -> Result<(), RunError> {
    let mut vm = VM::new();

    let mut func = Function::new();
    func.push_insts([
        Inst::Push(2.into()),
        Inst::Push(3.into()),
        Inst::Call(1),
        Inst::Halt,
    ]);

    let mut add_func = Function::new();
    add_func.push_insts([Inst::Add, Inst::Ret]);

    let mut program = Program::new();
    program.push_func(func);
    program.push_func(add_func);

    vm.execute(program).map_err(|e| {
        eprintln!("{:#?}", vm);
        e
    })?;

    dbg!(&vm);

    Ok(())
}
