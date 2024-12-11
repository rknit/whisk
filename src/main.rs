#![allow(dead_code)]

use module::Module;

pub mod ast;
pub mod ast_resolved;
pub mod cfg;
pub mod module;
pub mod symbol_table;
pub mod ty;

fn main() {
    let mut module = Module::new("test/test.wsk".into());
    module.parse_ast();
    module.resolve_ast();
    module.gen_cfg();
    module.display_cfg();
}
