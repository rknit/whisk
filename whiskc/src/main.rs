use whiskc::Module;

fn main() {
    let mut module = Module::new("test/test.wsk".into());
    module.parse_ast();
    let Some(ast) = module.resolve_ast() else {
        return;
    };
    dbg!(&ast);
    //module.gen_cfg();
    //module.display_cfg();
    module.codegen();
}
