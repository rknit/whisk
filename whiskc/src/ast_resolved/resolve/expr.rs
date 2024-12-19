use crate::{
    ast::{
        self,
        location::{Locatable, Located},
    },
    ast_resolved::{
        errors::{IdentResolveError, TypeResolveError, ValueResolveError},
        nodes::expr::{BinaryExpr, BlockExpr, CallExpr, Expr, IdentExpr, UnaryExpr},
        ControlFlow, ResolveContext,
    },
    symbol_table::Symbol,
    ty::{FuncType, PrimType, Type},
};

use ast::nodes::expr as ast_expr;

use super::stmt::StmtResolve;

pub type ResolvedExpr = (Option<Expr>, ControlFlow);

pub trait ExprResolve {
    fn resolve(&self, ctx: &mut ResolveContext) -> ResolvedExpr;
}

impl ExprResolve for ast_expr::Expr {
    fn resolve(&self, ctx: &mut ResolveContext) -> ResolvedExpr {
        match self {
            ast_expr::Expr::Integer(expr) => expr.resolve(ctx),
            ast_expr::Expr::Bool(expr) => expr.resolve(ctx),
            ast_expr::Expr::Identifier(expr) => expr.resolve(ctx),
            ast_expr::Expr::Unary(expr) => expr.resolve(ctx),
            ast_expr::Expr::Binary(expr) => expr.resolve(ctx),
            ast_expr::Expr::Grouped(expr) => expr.expr.resolve(ctx),
            ast_expr::Expr::Call(expr) => expr.resolve(ctx),
            ast_expr::Expr::Block(expr) => expr.resolve(ctx),
            _ => unimplemented!("expr resolve"),
        }
    }
}

impl ExprResolve for Located<i64> {
    fn resolve(&self, _ctx: &mut ResolveContext) -> ResolvedExpr {
        (
            Some(Expr::new(self.0, PrimType::Integer.into())),
            ControlFlow::Flow,
        )
    }
}

impl ExprResolve for Located<bool> {
    fn resolve(&self, _ctx: &mut ResolveContext) -> ResolvedExpr {
        (
            Some(Expr::new(self.0, PrimType::Bool.into())),
            ControlFlow::Flow,
        )
    }
}

impl ExprResolve for Located<String> {
    fn resolve(&self, ctx: &mut ResolveContext) -> ResolvedExpr {
        let Some(symbol) = ctx.get_symbol_by_name_mut(&self.0) else {
            ctx.push_error(IdentResolveError::UnknownIdentifier(self.clone()).into());
            return (None, ControlFlow::Flow);
        };
        symbol.push_ref(self.1);
        (
            Some(Expr::new(
                IdentExpr {
                    sym_id: symbol.get_id(),
                    ident: self.0.clone(),
                },
                symbol.get_type(),
            )),
            ControlFlow::Flow,
        )
    }
}

impl ExprResolve for ast_expr::UnaryExpr {
    fn resolve(&self, ctx: &mut ResolveContext) -> ResolvedExpr {
        let (expr, flow) = self.expr.resolve(ctx);
        if expr.is_none() || flow == ControlFlow::Return {
            return (expr, flow);
        }
        let expr = expr.unwrap();

        use crate::ast::parsing::token::Operator;
        let ty = match self.op.0 {
            Operator::Sub => {
                if !expr.get_type().is_numeric_ty() {
                    ctx.push_error(
                        TypeResolveError::NonNumericInUnaryOp(
                            self.op.0,
                            Located(expr.get_type(), self.get_location()),
                        )
                        .into(),
                    );
                    return (None, ControlFlow::Flow);
                }
                expr.get_type()
            }
            Operator::Not => {
                if expr.get_type() != PrimType::Bool.into() {
                    ctx.push_error(
                        TypeResolveError::NonBoolUsedInNotOp(Located(
                            expr.get_type(),
                            self.get_location(),
                        ))
                        .into(),
                    );
                }
                PrimType::Bool.into()
            }
            _ => unimplemented!("unary op '{}'", self.op.0),
        };

        (
            Some(Expr::new(
                UnaryExpr {
                    op: self.op.0,
                    expr: Box::new(expr),
                },
                ty,
            )),
            ControlFlow::Flow,
        )
    }
}

impl ExprResolve for ast_expr::BinaryExpr {
    fn resolve(&self, ctx: &mut ResolveContext) -> ResolvedExpr {
        let (left, flow) = self.left.resolve(ctx);
        if left.is_none() || flow == ControlFlow::Return {
            return (left, flow);
        }
        let left = left.unwrap();

        let (right, flow) = self.right.resolve(ctx);
        if right.is_none() || flow == ControlFlow::Return {
            return (right, flow);
        }
        let right = right.unwrap();

        let check_type_mismatch = |ctx: &mut ResolveContext| {
            if left.get_type() == right.get_type() {
                true
            } else {
                ctx.push_error(
                    TypeResolveError::TypeMismatchInBinaryOp {
                        op: Located(self.op.0, self.get_location()),
                        left_ty: left.get_type(),
                        right_ty: right.get_type(),
                    }
                    .into(),
                );
                false
            }
        };

        use crate::ast::parsing::token::Operator;
        let ty = match self.op.0 {
            Operator::Add | Operator::Sub => {
                if !left.get_type().is_numeric_ty() {
                    ctx.push_error(
                        TypeResolveError::NonNumericTypeInBinaryOp {
                            op: self.op.clone(),
                            ty: Located(left.get_type(), self.left.get_location()),
                        }
                        .into(),
                    );
                    return (None, ControlFlow::Flow);
                }
                if !right.get_type().is_numeric_ty() {
                    ctx.push_error(
                        TypeResolveError::NonNumericTypeInBinaryOp {
                            op: self.op.clone(),
                            ty: Located(right.get_type(), self.right.get_location()),
                        }
                        .into(),
                    );
                    return (None, ControlFlow::Flow);
                }

                if !check_type_mismatch(ctx) {
                    return (None, ControlFlow::Flow);
                }
                left.get_type()
            }
            Operator::And | Operator::Or => {
                if left.get_type() != PrimType::Bool.into() {
                    ctx.push_error(
                        TypeResolveError::UnexpectedTypeInBinaryOp {
                            op: self.op.clone(),
                            expect_type: PrimType::Bool.into(),
                            actual_type: Located(left.get_type(), self.left.get_location()),
                        }
                        .into(),
                    );
                }
                if right.get_type() != PrimType::Bool.into() {
                    ctx.push_error(
                        TypeResolveError::UnexpectedTypeInBinaryOp {
                            op: self.op.clone(),
                            expect_type: PrimType::Bool.into(),
                            actual_type: Located(right.get_type(), self.left.get_location()),
                        }
                        .into(),
                    );
                }
                PrimType::Bool.into()
            }
            Operator::Equal
            | Operator::NotEqual
            | Operator::Less
            | Operator::LessEqual
            | Operator::Greater
            | Operator::GreaterEqual => {
                if !left.get_type().is_ord_ty() {
                    ctx.push_error(
                        TypeResolveError::UnorderedTypeInBinaryOp {
                            op: self.op.clone(),
                            ty: Located(left.get_type(), self.left.get_location()),
                        }
                        .into(),
                    );
                }
                if !right.get_type().is_ord_ty() {
                    ctx.push_error(
                        TypeResolveError::UnorderedTypeInBinaryOp {
                            op: self.op.clone(),
                            ty: Located(right.get_type(), self.right.get_location()),
                        }
                        .into(),
                    );
                }
                check_type_mismatch(ctx);
                PrimType::Bool.into()
            }
            _ => unimplemented!("binary op '{}'", self.op.0),
        };

        (
            Some(Expr::new(
                BinaryExpr {
                    op: self.op.0,
                    left: Box::new(left),
                    right: Box::new(right),
                },
                ty,
            )),
            ControlFlow::Flow,
        )
    }
}

impl ExprResolve for ast_expr::CallExpr {
    fn resolve(&self, ctx: &mut ResolveContext) -> ResolvedExpr {
        let (callee, flow) = self.callee.resolve(ctx);
        if callee.is_none() || flow == ControlFlow::Return {
            return (callee, flow);
        }
        let callee = callee.unwrap();

        let Type::Function(FuncType(func_sym_id)) = callee.get_type() else {
            ctx.push_error(
                TypeResolveError::CallOnNonFunctionType(Located(
                    callee.get_type(),
                    self.get_location(),
                ))
                .into(),
            );
            return (None, ControlFlow::Flow);
        };

        let (expect_params, ret_ty) = {
            let Symbol::Function(func) = ctx.get_symbol(func_sym_id).expect("valid function id")
            else {
                panic!("symbol is not a function");
            };

            (
                func.get_params().clone(), /* :( */
                func.get_return_type(),
            )
        };

        if expect_params.len() != self.args.items.len() {
            ctx.push_error(
                ValueResolveError::ArgumentCountMismatch {
                    func_ty: Located(callee.get_type(), self.get_location()),
                    expect_count: expect_params.len(),
                    actual_count: self.args.items.len(),
                }
                .into(),
            );
        }

        let mut args = Vec::new();

        for ((i, expect_param), ast_arg) in expect_params.iter().enumerate().zip(&self.args.items) {
            let (arg, flow) = ast_arg.resolve(ctx);
            if flow == ControlFlow::Return {
                return (arg, flow);
            }
            let Some(arg) = arg else {
                continue;
            };

            if expect_param.1 == arg.get_type() {
                args.push(arg);
                continue;
            }

            ctx.push_error(
                TypeResolveError::ArgumentTypeMismatch {
                    func_ty: Located(callee.get_type(), self.get_location()),
                    argument_index: i,
                    expect_type: expect_param.1,
                    actual_type: Located(arg.get_type(), ast_arg.get_location()),
                }
                .into(),
            );
        }

        (
            Some(Expr::new(
                CallExpr {
                    callee: Box::new(callee),
                    args,
                },
                ret_ty,
            )),
            ControlFlow::Flow,
        )
    }
}

impl ExprResolve for ast_expr::BlockExpr {
    fn resolve(&self, ctx: &mut ResolveContext) -> ResolvedExpr {
        let mut stmts = Vec::new();
        let table_id = ctx.push_local();

        for ast_stmt in &self.stmts {
            let (stmt, flow) = ast_stmt.resolve(ctx);
            if let Some(stmt) = stmt {
                stmts.push(stmt);
            }
            match flow {
                ControlFlow::Flow => (),
                ControlFlow::Return => {
                    ctx.pop_local();
                    return (None, ControlFlow::Return);
                }
            };
        }

        let eval_expr = if let Some(eval_expr) = &self.eval_expr {
            let (expr, flow) = eval_expr.resolve(ctx);
            if expr.is_none() || flow == ControlFlow::Return {
                return (expr, flow);
            }
            Some(Box::new(expr.unwrap()))
        } else {
            None
        };
        let eval_ty = eval_expr
            .as_ref()
            .map(|v| v.get_type())
            .unwrap_or(PrimType::Unit.into());

        ctx.pop_local();
        (
            Some(Expr::new(
                BlockExpr {
                    table_id,
                    stmts,
                    eval_expr,
                },
                eval_ty,
            )),
            ControlFlow::Flow,
        )
    }
}
