use whiskc::Module;

fn main() {
    let mut module = Module::new("test/test.wsk".into());
    module.parse_ast();
    module.resolve_ast();
    module.gen_cfg();
    module.display_cfg();
}
