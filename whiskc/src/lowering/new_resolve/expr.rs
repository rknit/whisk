use crate::{
    ast::{location::Located, nodes as ast, parsing::token::Operator},
    lowering::{
        new_resolve::Flow,
        nodes::expr::{
            BinaryExpr, BlockExpr, CallExpr, Expr, ExprKind, FuncIdentExpr, IfExpr, LoopExpr,
            ReturnExpr, UnaryExpr, VarIdentExpr,
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
            ast::expr::Expr::Unary(v) => v.resolve(ctx, ()),
            ast::expr::Expr::Binary(v) => v.resolve(ctx, ()),
            ast::expr::Expr::Grouped(v) => v.expr.resolve(ctx, ()),
            ast::expr::Expr::Call(v) => v.resolve(ctx, ()),
            ast::expr::Expr::Block(v) => v.resolve(ctx, ()),
            ast::expr::Expr::Return(v) => v.resolve(ctx, ()),
            ast::expr::Expr::If(v) => v.resolve(ctx, ()),
            ast::expr::Expr::Loop(v) => v.resolve(ctx, ()),
        }
    }
}

impl Resolve<(), FlowObj<Expr>> for ast::expr::BinaryExpr {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> FlowObj<Expr> {
        let FlowObj {
            value: left,
            flow: left_flow,
        } = self.left.resolve(ctx, ());

        let FlowObj {
            value: right,
            flow: right_flow,
        } = self.right.resolve(ctx, ());

        let merged_flow = left_flow & right_flow;
        let (Some(left), Some(right)) = (left, right) else {
            return FlowObj::none(merged_flow);
        };

        let check_ty_equal = || {
            if left.ty != right.ty {
                todo!("report error")
            }
        };
        let check_ty_num = |expr: &Expr| {
            if expr.ty != ctx.table.common_type().int {
                todo!("report error")
            }
        };
        let check_ty_bool = |expr: &Expr| {
            if expr.ty != ctx.table.common_type().bool {
                todo!("report error")
            }
        };

        let op_ty = match self.op.0 {
            Operator::Assign => {
                // TODO: check if expr is assignable
                check_ty_equal();
                ctx.table.common_type().unit
            }
            Operator::Add | Operator::Sub => {
                check_ty_num(&left);
                check_ty_num(&right);
                check_ty_equal();
                left.ty
            }
            Operator::And | Operator::Or => {
                check_ty_bool(&left);
                check_ty_bool(&right);
                ctx.table.common_type().bool
            }
            Operator::Equal
            | Operator::NotEqual
            | Operator::Less
            | Operator::LessEqual
            | Operator::Greater
            | Operator::GreaterEqual => {
                check_ty_num(&left);
                check_ty_num(&right);
                check_ty_equal();
                ctx.table.common_type().bool
            }
            _ => unimplemented!(),
        };

        FlowObj::new(
            Expr {
                kind: BinaryExpr {
                    op: self.op.0,
                    left: Box::new(left),
                    right: Box::new(right),
                }
                .into(),
                ty: op_ty,
            },
            merged_flow,
        )
    }
}

impl Resolve<(), FlowObj<Expr>> for ast::expr::UnaryExpr {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> FlowObj<Expr> {
        let FlowObj { value, flow } = self.expr.resolve(ctx, ());
        let Some(value) = value else {
            return FlowObj::none(flow);
        };
        if flow != Flow::Continue {
            return FlowObj::new(value, flow);
        };

        let op_ty = match self.op.0 {
            Operator::Sub => {
                if value.ty != ctx.table.common_type().int {
                    todo!("report error")
                }
                ctx.table.common_type().int
            }
            Operator::Not => {
                if value.ty != ctx.table.common_type().bool {
                    todo!("report error")
                }
                ctx.table.common_type().bool
            }
            _ => unimplemented!(),
        };

        FlowObj::new(
            Expr {
                kind: UnaryExpr {
                    op: self.op.0,
                    expr: Box::new(value),
                }
                .into(),
                ty: op_ty,
            },
            flow,
        )
    }
}

impl Resolve<(), FlowObj<Expr>> for ast::expr::IfExpr {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> FlowObj<Expr> {
        let FlowObj { value, flow } = self.cond.resolve(ctx, ());
        let Some(cond) = value else {
            return FlowObj::none(flow);
        };
        if flow != Flow::Continue {
            return FlowObj::new(cond, flow);
        }

        if cond.ty != ctx.table.common_type().bool {
            todo!("report error")
        }

        let FlowObj {
            value: then_body,
            flow: then_flow,
        } = self.then.resolve(ctx, ());

        if let Some(else_) = &self.else_expr {
            let FlowObj {
                value: else_body,
                flow: else_flow,
            } = else_.body.resolve(ctx, ());

            let merged_flow = then_flow & else_flow;

            let (Some(then), Some(else_)) = (then_body, else_body) else {
                // TODO: is this the right behavior?
                return FlowObj::none(merged_flow);
            };

            if then.ty != else_.ty {
                todo!("report error")
            }
            let if_ty = then.ty;
            let (ExprKind::Block(then), ExprKind::Block(else_)) = (then.kind, else_.kind) else {
                unreachable!()
            };

            FlowObj::new(
                Expr {
                    kind: IfExpr {
                        cond: Box::new(cond),
                        then,
                        else_: Some(else_),
                    }
                    .into(),
                    ty: if_ty,
                },
                merged_flow,
            )
        } else {
            let Some(then_body) = then_body else {
                // TODO: is this the right behavior?
                return FlowObj::none(then_flow);
            };
            if then_body.ty != ctx.table.common_type().unit {
                todo!("report error")
            }
            let ExprKind::Block(then) = then_body.kind else {
                unreachable!()
            };
            FlowObj::new(
                Expr {
                    kind: IfExpr {
                        cond: Box::new(cond),
                        then,
                        else_: None,
                    }
                    .into(),
                    ty: ctx.table.common_type().unit,
                },
                then_flow,
            )
        }
    }
}

impl Resolve<(), FlowObj<Expr>> for ast::expr::LoopExpr {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> FlowObj<Expr> {
        let FlowObj { value, flow } = self.body.resolve(ctx, ());
        let Some(body) = value else {
            return FlowObj::none(flow);
        };
        if flow != Flow::Continue {
            return FlowObj::new(body, flow);
        }
        let ExprKind::Block(block_expr) = body.kind else {
            unreachable!()
        };
        FlowObj::cont(Expr {
            kind: LoopExpr { body: block_expr }.into(),
            // always return the Never type for now, as there is no Break/Continue expr yet.
            ty: ctx.table.common_type().never,
        })
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
                kind: ReturnExpr {
                    expr: Some(Box::new(value)),
                }
                .into(),
                ty: ctx.table.common_type().never,
            })
        } else {
            let sym = ctx.get_func_id().sym(ctx.table);
            if sym.get_return_type() != ctx.table.common_type().unit {
                todo!("report error");
            }
            FlowObj::brk(Expr {
                kind: ReturnExpr { expr: None }.into(),
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
                kind: CallExpr {
                    caller: Box::new(caller),
                    args,
                }
                .into(),
            },
            result_flow,
        )
    }
}

impl Resolve<(), FlowObj<Expr>> for Located<String> {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> FlowObj<Expr> {
        if let Some(var) = ctx.table.get_variable_by_name_mut(ctx.get_block(), &self.0) {
            FlowObj::cont(Expr {
                kind: VarIdentExpr { id: var.get_id() }.into(),
                ty: var.get_type(),
            })
        } else if let Some(func) = ctx.table.get_function_by_name_mut(&self.0) {
            FlowObj::cont(Expr {
                kind: FuncIdentExpr { id: func.get_id() }.into(),
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
                    kind: BlockExpr {
                        block_id: bid,
                        stmts,
                        eval_expr: None,
                    }
                    .into(),
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
                kind: BlockExpr {
                    block_id: bid,
                    stmts,
                    eval_expr: eval_expr.and_then(|v| v.value.map(Box::new)),
                }
                .into(),
                ty: eval_expr_ty,
            },
            result_flow,
        )
    }
}
