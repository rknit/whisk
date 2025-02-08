use crate::{ast::nodes as ast, lowering::nodes::stmt::Stmt};

use super::{FlowObj, Resolve, ResolveContext};

impl Resolve<(), FlowObj<Stmt>> for ast::stmt::Stmt {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> FlowObj<Stmt> {
        match self {
            ast::stmt::Stmt::Expr(_) => todo!(),
            ast::stmt::Stmt::Let(_) => todo!(),
        }
    }
}
