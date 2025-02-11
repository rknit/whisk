use crate::{
    ast::{location::Located, nodes as ast},
    lowering::{
        new_resolve::Flow,
        nodes::expr::{
            BlockExpr, CallExpr, Expr, ExprKind, FuncIdentExpr, ReturnExpr, VarIdentExpr,
        },
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
            ast::expr::Expr::Identifier(v) => v.resolve(ctx, ()),
            ast::expr::Expr::Unary(_) => todo!(),
            ast::expr::Expr::Binary(_) => todo!(),
            ast::expr::Expr::Grouped(v) => v.expr.resolve(ctx, ()),
            ast::expr::Expr::Call(v) => v.resolve(ctx, ()),
            ast::expr::Expr::Block(v) => v.resolve(ctx, ()),
            ast::expr::Expr::Return(v) => v.resolve(ctx, ()),
            ast::expr::Expr::If(_) => todo!(),
            ast::expr::Expr::Loop(_) => todo!(),
        }
    }
}

impl Resolve<(), FlowObj<Expr>> for ast::expr::ReturnExpr {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> FlowObj<Expr> {
        let expr = self.expr.as_ref().map(|v| v.resolve(ctx, ()));
        if let Some(FlowObj { value, flow }) = expr {
            let Some(value) = value else {
                return FlowObj::none(flow);
            };
            if flow != Flow::Continue {
                return FlowObj::new(value, flow);
            }
            let sym = ctx.get_func_id().sym(ctx.table);
            if value.ty != sym.get_return_type() {
                todo!("report error");
            }
            FlowObj::brk(Expr {
                kind: ExprKind::Return(ReturnExpr {
                    expr: Some(Box::new(value)),
                }),
                ty: ctx.table.common_type().never,
            })
        } else {
            let sym = ctx.get_func_id().sym(ctx.table);
            if sym.get_return_type() != ctx.table.common_type().unit {
                todo!("report error");
            }
            FlowObj::brk(Expr {
                kind: ExprKind::Return(ReturnExpr { expr: None }),
                ty: ctx.table.common_type().never,
            })
        }
    }
}

impl Resolve<(), FlowObj<Expr>> for ast::expr::CallExpr {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> FlowObj<Expr> {
        let FlowObj {
            value,
            flow: mut result_flow,
        } = self.caller.resolve(ctx, ());
        let Some(caller) = value else {
            // assumed the earlier resolve call had already reported the error.
            return FlowObj::none(result_flow);
        };
        let ExprKind::FuncIdent(FuncIdentExpr { id: fid }) = caller.kind else {
            todo!("report error")
        };

        let sym = fid.sym(ctx.table);
        if self.args.items.len() != sym.params().len() {
            todo!("report error");
        }

        let mut args = Vec::new();
        for (ast_arg, param_id) in self.args.items.iter().zip(sym.params().clone()) {
            let FlowObj { value, flow } = ast_arg.resolve(ctx, ());
            let Some(arg) = value else {
                // assumed the resolve called had already reported the error.
                continue;
            };
            if param_id.sym(ctx.table).get_type() != arg.ty {
                todo!("report error");
            }
            args.push(arg);
            result_flow = flow;
            if result_flow != Flow::Continue {
                // stop evaluating the subsequence arguments if the control flow won't reach them.
                break;
            }
        }

        FlowObj::new(
            Expr {
                ty: fid.sym(ctx.table).get_return_type(),
                kind: ExprKind::Call(CallExpr {
                    caller: Box::new(caller),
                    args,
                }),
            },
            result_flow,
        )
    }
}

impl Resolve<(), FlowObj<Expr>> for Located<String> {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> FlowObj<Expr> {
        if let Some(var) = ctx.table.get_variable_by_name_mut(ctx.get_block(), &self.0) {
            FlowObj::cont(Expr {
                kind: ExprKind::VarIdent(VarIdentExpr { id: var.get_id() }),
                ty: var.get_type(),
            })
        } else if let Some(func) = ctx.table.get_function_by_name_mut(&self.0) {
            FlowObj::cont(Expr {
                kind: ExprKind::FuncIdent(FuncIdentExpr { id: func.get_id() }),
                ty: func.get_return_type(),
            })
        } else if ctx.table.get_type_by_name_mut(&self.0).is_some() {
            todo!("report error")
        } else {
            todo!("report error")
        }
    }
}

impl Resolve<(), FlowObj<Expr>> for ast::expr::BlockExpr {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> FlowObj<Expr> {
        let bid = ctx.table.new_block(ctx.get_func_id());
        {
            let parent_block = ctx.get_block();
            bid.sym(ctx.table).set_parent_block(parent_block);
        }
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
