use std::env;
use std::fs;

use wsk_vm::program::Program;
use wsk_vm::RunError;
use wsk_vm::VM;

fn main() -> Result<(), RunError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(RunError::MissingSourcefile);
    }

    let bytes = fs::read(args[1].clone()).unwrap();
    let program = Program::from_bytes(&bytes)?;
    println!("{}", program);

    let mut vm = VM::new();
    vm.execute(program).map_err(|e| {
        eprintln!("{:#?}", vm);
        e
    })?;

    println!("{:#?}", vm);

    Ok(())
}
