use std::env;

use whiskc::compile::{self, CompileSwitch};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("whiskc: expected path to .wsk sourcefile.");
        return;
    }

    compile::compile(
        args[1].clone().into(),
        CompileSwitch {
            do_parse_ast: true,
            debug_ast: true,
            do_resolve_module: true,
            print_module: true,
            do_codegen: true,
        },
    )
}
