use crate::{
    ast::AST,
    symbol::{FuncId, SymbolTable},
};

use super::{errors::ResolveError, Module};

mod ast;
mod expr;
mod func;
mod item;
mod stmt;
mod ty;

pub fn resolve(ast: &AST) -> Result<Module, Vec<ResolveError>> {
    let mut module = Module {
        sym_table: SymbolTable::default(),
        items: vec![],
    };

    let mut ctx = ResolveContext {
        table: &mut module.sym_table,
        errors: vec![],
        current_fid: None,
    };

    module.items = ast.resolve(&mut ctx, ());
    dbg!(&ctx);

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
    current_fid: Option<FuncId>,
}
impl ResolveContext<'_> {
    pub fn get_func_id(&self) -> FuncId {
        self.current_fid.unwrap()
    }

    pub fn _error(&mut self, e: impl Into<ResolveError>) {
        self.errors.push(e.into());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Flow {
    /// control flow continues to flow down the current path.
    Continue,
    /// control flow diverges from the current path.
    Break,
}

struct FlowObj<T> {
    pub value: Option<T>,
    pub flow: Flow,
}
impl<T> FlowObj<T> {
    pub fn new(t: T, flow: Flow) -> Self {
        Self {
            value: Some(t),
            flow,
        }
    }

    pub fn cont(t: T) -> Self {
        Self {
            value: Some(t),
            flow: Flow::Continue,
        }
    }

    pub fn cont_none() -> Self {
        Self {
            value: None,
            flow: Flow::Continue,
        }
    }

    pub fn brk(t: T) -> Self {
        Self {
            value: Some(t),
            flow: Flow::Break,
        }
    }

    pub fn brk_none() -> Self {
        Self {
            value: None,
            flow: Flow::Break,
        }
    }
}
