use crate::{
    ast::nodes as ast,
    lowering::{
        new_resolve::Flow,
        nodes::expr::{BlockExpr, Expr, ExprKind},
    },
};

use super::{FlowObj, Resolve, ResolveContext};

impl Resolve<(), FlowObj<Expr>> for ast::expr::Expr {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> FlowObj<Expr> {
        match self {
            ast::expr::Expr::Unit(_) => FlowObj::cont(Expr {
                kind: ExprKind::Unit,
                ty: ctx.table.common_type().unit,
            }),
            ast::expr::Expr::Integer(v) => FlowObj::cont(Expr {
                kind: ExprKind::Integer(v.0),
                ty: ctx.table.common_type().int,
            }),
            ast::expr::Expr::Bool(v) => FlowObj::cont(Expr {
                kind: ExprKind::Bool(v.0),
                ty: ctx.table.common_type().bool,
            }),
            ast::expr::Expr::Identifier(_) => todo!(),
            ast::expr::Expr::Unary(_) => todo!(),
            ast::expr::Expr::Binary(_) => todo!(),
            ast::expr::Expr::Grouped(v) => v.expr.resolve(ctx, ()),
            ast::expr::Expr::Call(_) => todo!(),
            ast::expr::Expr::Block(v) => v.resolve(ctx, ()),
            ast::expr::Expr::Return(_) => todo!(),
            ast::expr::Expr::If(_) => todo!(),
            ast::expr::Expr::Loop(_) => todo!(),
        }
    }
}

impl Resolve<(), FlowObj<Expr>> for ast::expr::BlockExpr {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> FlowObj<Expr> {
        let bid = ctx.table.new_block(ctx.get_func_id());
        ctx.push_block(bid);

        let mut stmts = Vec::new();
        let mut result_flow = Flow::Continue;
        for stmt_ast in &self.stmts {
            let FlowObj { value: stmt, flow } = stmt_ast.resolve(ctx, ());
            result_flow = flow;
            if result_flow == Flow::Break {
                break;
            }
            let Some(stmt) = stmt else {
                continue;
            };
            stmts.push(stmt);
        }

        if result_flow == Flow::Break {
            ctx.pop_block();

            return FlowObj::new(
                Expr {
                    kind: ExprKind::Block(BlockExpr {
                        block_id: bid,
                        stmts,
                        eval_expr: None,
                    }),
                    ty: ctx.table.common_type().never,
                },
                result_flow,
            );
        }

        let eval_expr = self.eval_expr.as_ref().map(|v| v.resolve(ctx, ()));
        let eval_expr_ty = eval_expr
            .as_ref()
            .and_then(|v| v.value.as_ref().map(|v| v.ty))
            .unwrap_or(ctx.table.common_type().unit);
        if let Some(expr) = &eval_expr {
            result_flow = expr.flow;
        }

        ctx.pop_block();

        FlowObj::new(
            Expr {
                kind: ExprKind::Block(BlockExpr {
                    block_id: bid,
                    stmts,
                    eval_expr: eval_expr.and_then(|v| v.value.map(Box::new)),
                }),
                ty: eval_expr_ty,
            },
            result_flow,
        )
    }
}
