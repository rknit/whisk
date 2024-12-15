use whiskc::Module;

fn main() {
    let mut module = Module::new("test/test.wsk".into());
    module.parse_ast();
    let r = module.resolve_ast();
    dbg!(&r);
    //module.gen_cfg();
    //module.display_cfg();
}
