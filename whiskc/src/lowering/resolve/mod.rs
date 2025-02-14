use std::ops::BitAnd;

use crate::{
    ast::AST,
    symbol::{BlockId, FuncId, SymbolTable},
};

use super::{errors::ResolveError, Module};

mod ast;
mod expr;
mod func;
mod item;
mod stmt;
mod ty;

pub fn resolve(ast: &AST, module_name: String) -> Result<Module, Vec<ResolveError>> {
    let mut module = Module {
        sym_table: SymbolTable::default(),
        name: module_name,
        items: vec![],
    };

    let mut ctx = ResolveContext::new(&mut module.sym_table);
    module.items = ast.resolve(&mut ctx, ());
    // dbg!(&ctx);

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
    blocks: Vec<BlockId>,
}
impl<'a> ResolveContext<'a> {
    pub fn new(table: &'a mut SymbolTable) -> Self {
        Self {
            table,
            errors: vec![],
            current_fid: None,
            blocks: vec![],
        }
    }

    pub fn set_func_id(&mut self, fid: FuncId) {
        assert!(
            self.current_fid.is_none(),
            "must unset func id before setting a new one"
        );
        self.current_fid = Some(fid);
    }

    pub fn unset_func_id(&mut self) {
        assert!(self.current_fid.is_some(), "no func id to unset");
        assert!(self.blocks.is_empty(), "not all blocks are popped");
        self.current_fid = None;
    }

    pub fn get_func_id(&self) -> FuncId {
        self.current_fid.unwrap()
    }

    pub fn push_block(&mut self, block: BlockId) {
        assert!(self.current_fid.is_some(), "no func id set");
        self.blocks.push(block);
    }

    pub fn pop_block(&mut self) {
        self.blocks.pop().unwrap();
    }

    pub fn get_block(&self) -> BlockId {
        self.blocks.last().copied().unwrap()
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

impl BitAnd for Flow {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        if self == Flow::Break && rhs == Flow::Break {
            Flow::Break
        } else {
            Flow::Continue
        }
    }
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

    pub fn none(flow: Flow) -> Self {
        Self { value: None, flow }
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

    pub fn _brk_none() -> Self {
        Self {
            value: None,
            flow: Flow::Break,
        }
    }
}
