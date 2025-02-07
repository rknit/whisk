use crate::{ast::AST, symbol::SymbolTable};

use super::{errors::ResolveError, Module};

mod ast;
mod func;
mod item;

pub fn resolve(ast: &AST) -> Result<Module, Vec<ResolveError>> {
    let mut module = Module {
        sym_table_old: todo!(),
        sym_table: SymbolTable::default(),
        items: vec![],
    };

    let mut ctx = ResolveContext {
        table: &mut module.sym_table,
        errors: vec![],
    };

    if !ctx.errors.is_empty() {
        Err(ctx.errors)
    } else {
        Ok(module)
    }
}

/// Record trait records the symbols into the symbol table, but does not resolve them immediately.
/// It is mainly for pre-declaring global items, so that they can refer to other global items that are declared later in
/// the source code.
trait Record<Arg = (), R = ()> {
    fn record(&self, ctx: &mut ResolveContext, arg: Arg) -> R;
}

/// Resolve trait should validate the semantic of the AST and its nodes, and report errors if found
/// any.
trait Resolve<Arg = (), R = ()> {
    fn resolve(&self, ctx: &mut ResolveContext, arg: Arg) -> R;
}

#[derive(Debug)]
struct ResolveContext<'md> {
    table: &'md mut SymbolTable,
    errors: Vec<ResolveError>,
}
impl ResolveContext<'_> {
    pub fn error(&mut self, e: impl Into<ResolveError>) {
        self.errors.push(e.into());
    }
}
