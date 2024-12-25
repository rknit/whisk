use std::fs::File;

use whiskc::new_ast::lexer::CharReader;

fn main() {
    let f = File::open("./test/test.wsk").unwrap();
    let mut rd = CharReader::new(f);
    while !rd.is_eof() {
        print!("{}", rd.next_char());
    }

    // let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     eprintln!("whiskc: expected path to .wsk sourcefile.");
    //     return;
    // }
    //
    // let mut module = Module::new(args[1].clone().into());
    // let Some(_ast) = module.parse_ast() else {
    //     return;
    // };
    // //dbg!(&ast);
    // let Some(_ast) = module.resolve_ast() else {
    //     return;
    // };
    // //dbg!(&ast);
    // //module.gen_cfg();
    // //module.display_cfg();
    // module.codegen();
}
