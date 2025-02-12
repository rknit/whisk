use crate::{
    ast::nodes as ast,
    lowering::nodes::stmt::{ExprStmt, LetStmt, Stmt},
};

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
            ast::stmt::Stmt::Expr(v) => v.resolve(ctx, ()).map_stmt(Stmt::Expr),
            ast::stmt::Stmt::Let(v) => v.resolve(ctx, ()).map_stmt(Stmt::Let),
        }
    }
}

impl Resolve<(), FlowObj<ExprStmt>> for ast::stmt::ExprStmt {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> FlowObj<ExprStmt> {
        let FlowObj { value, flow } = self.expr.resolve(ctx, ());
        FlowObj {
            value: value.map(|v| ExprStmt { expr: v }),
            flow,
        }
    }
}

impl Resolve<(), FlowObj<LetStmt>> for ast::stmt::LetStmt {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> FlowObj<LetStmt> {
        let FlowObj { value, flow } = self.value.resolve(ctx, ());
        let Some(value) = value else {
            return FlowObj::none(flow);
        };

        let mut var_ty = value.ty;
        let anno_ty = self.ty.as_ref().and_then(|v| v.resolve(ctx, ()));
        if let Some(anno_ty) = anno_ty {
            if anno_ty != value.ty {
                todo!("report error")
            }
            var_ty = anno_ty;
        }

        let Some(var_id) = ctx.table.new_variable(self.name.0.clone(), ctx.get_block()) else {
            todo!("report error");
        };
        var_id.sym_mut(ctx.table).ty = var_ty;

        FlowObj::new(LetStmt { var_id, value }, flow)
    }
}
