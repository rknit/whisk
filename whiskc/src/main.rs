use std::env;

use whiskc::Module;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("whiskc: expected path to .wsk sourcefile.");
        return;
    }

    let mut module = Module::new(args[1].clone().into());
    let Some(ast) = module.parse_ast() else {
        return;
    };
    dbg!(&ast);
    module.resolve_ast();
    let Some(_ast) = module.run_passes() else {
        return;
    };
    // dbg!(&ast);
    //module.gen_cfg();
    //module.display_cfg();
    module.codegen();
}
