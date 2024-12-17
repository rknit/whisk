use nodes::program::Program;

use crate::{ast_resolved::ResolvedAST, symbol_table::SymbolTable};

pub mod nodes;

struct Context<'a> {
    pub sym_table: &'a SymbolTable,
    pub depth: usize,
}

trait Codegen<R> {
    fn codegen(&self, ctx: &mut Context) -> R;
}

pub fn codegen_wsk_vm(ast: &ResolvedAST) -> Program {
    todo!()
}
