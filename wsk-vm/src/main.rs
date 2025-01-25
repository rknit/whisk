use wsk_vm::new_vm::{
    abi::reg,
    inst::{Inst, RunError},
    program::{Function, Program},
    vm::VM,
};

fn main() -> Result<(), RunError> {
    let mut prog = Program::new(0);
    prog.add_func(Function::from_insts([
        Inst::MOVV {
            dest: reg(0),
            value: 5.into(),
        },
        Inst::PUSH { reg: reg(0) },
        Inst::POP { dest: reg(6) },
        Inst::ADD {
            dest: reg(1),
            p0: reg(6),
            p1: reg(6),
        },
        Inst::HLT,
    ]));

    println!("{}", prog);

    let mut vm = VM::default();
    vm.execute(&prog)?;

    println!("{:#?}", vm);

    /*
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(RunError::MissingSourcefile);
    }

    let bytes = fs::read(args[1].clone()).unwrap();
    let program = Program::from_bytes(&bytes)?;
    println!("{}", program);

    let mut vm = VM::default();
    vm.execute(program).map_err(|e| {
        eprintln!("{:#?}", vm);
        e
    })?;

    println!("{:#?}", vm);
    */

    Ok(())
}
