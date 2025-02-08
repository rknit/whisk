use crate::{ast::nodes as ast, lowering::nodes::stmt::Stmt};

use super::{FlowObj, Resolve, ResolveContext};

impl<T> FlowObj<T> {
    pub fn map_stmt<F>(self, f: F) -> FlowObj<Stmt>
    where
        F: FnOnce(T) -> Stmt,
    {
        FlowObj {
            value: self.value.map(f),
            flow: self.flow,
        }
    }
}

impl Resolve<(), FlowObj<Stmt>> for ast::stmt::Stmt {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> FlowObj<Stmt> {
        match self {
            ast::stmt::Stmt::Expr(_) => todo!(),
            ast::stmt::Stmt::Let(_) => todo!(),
        }
    }
}
