use std::env;

use whiskc::Compilation;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("whiskc: expected path to .wsk sourcefile.");
        return;
    }

    let mut compl = Compilation::new(args[1].clone().into());
    let Some(_ast) = compl.parse_ast() else {
        return;
    };
    // dbg!(&ast);

    let Some(_ast) = compl.resolve_module() else {
        return;
    };
    // dbg!(&ast);

    compl.codegen();
}
