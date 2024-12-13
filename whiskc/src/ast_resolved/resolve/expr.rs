use crate::{
    ast::{
        self,
        location::{Locatable, Located},
    },
    ast_resolved::{
        errors::{IdentResolveError, TypeResolveError, ValueResolveError},
        nodes::expr::{BinaryExpr, CallExpr, Expr, ExprKind, UnaryExpr},
        Resolve, ResolveContext,
    },
    symbol_table::Symbol,
    ty::{FuncType, PrimType, Type},
};

use ast::nodes::expr as ast_expr;

impl Resolve<Expr> for ast_expr::Expr {
    fn resolve(&self, ctx: &mut ResolveContext) -> Option<Expr> {
        match self {
            ast_expr::Expr::Integer(expr) => expr.resolve(ctx),
            ast_expr::Expr::Bool(expr) => expr.resolve(ctx),
            ast_expr::Expr::Identifier(expr) => expr.resolve(ctx),
            ast_expr::Expr::Unary(expr) => expr.resolve(ctx),
            ast_expr::Expr::Binary(expr) => expr.resolve(ctx),
            ast_expr::Expr::Grouped(expr) => expr.expr.resolve(ctx),
            ast_expr::Expr::Call(expr) => expr.resolve(ctx),
            _ => unimplemented!("expr resolve"),
        }
    }
}

impl Resolve<Expr> for Located<i64> {
    fn resolve(&self, _ctx: &mut ResolveContext) -> Option<Expr> {
        Some(Expr {
            kind: ExprKind::Integer(self.0),
            ty: PrimType::Integer.into(),
        })
    }
}

impl Resolve<Expr> for Located<bool> {
    fn resolve(&self, _ctx: &mut ResolveContext) -> Option<Expr> {
        Some(Expr {
            kind: ExprKind::Bool(self.0),
            ty: PrimType::Bool.into(),
        })
    }
}

impl Resolve<Expr> for Located<String> {
    fn resolve(&self, ctx: &mut ResolveContext) -> Option<Expr> {
        let Some(symbol) = ctx.get_symbol_by_name(&self.0) else {
            ctx.push_error(IdentResolveError::UnknownIdentifier(self.clone()).into());
            return None;
        };
        Some(Expr {
            kind: ExprKind::Identifier(self.0.clone()),
            ty: symbol.get_type(),
        })
    }
}

impl Resolve<Expr> for ast_expr::UnaryExpr {
    fn resolve(&self, ctx: &mut ResolveContext) -> Option<Expr> {
        let expr = self.expr.resolve(ctx)?;

        use crate::ast::parsing::token::Operator;
        Some(match self.op.0 {
            Operator::Sub => {
                if !expr.ty.is_numeric_ty() {
                    ctx.push_error(
                        TypeResolveError::NonNumericInUnaryOp(
                            self.op.0,
                            Located(expr.ty, self.get_location()),
                        )
                        .into(),
                    );
                    return None;
                }
                Expr {
                    ty: expr.ty,
                    kind: ExprKind::Unary(UnaryExpr {
                        op: self.op.0,
                        expr: Box::new(expr),
                    }),
                }
            }
            Operator::Not => {
                if expr.ty != PrimType::Bool.into() {
                    ctx.push_error(
                        TypeResolveError::NonBoolUsedInNotOp(Located(expr.ty, self.get_location()))
                            .into(),
                    );
                }
                Expr {
                    kind: ExprKind::Unary(UnaryExpr {
                        op: self.op.0,
                        expr: Box::new(expr),
                    }),
                    ty: PrimType::Bool.into(),
                }
            }
            _ => unimplemented!("unary op '{}'", self.op.0),
        })
    }
}

impl Resolve<Expr> for ast_expr::BinaryExpr {
    fn resolve(&self, ctx: &mut ResolveContext) -> Option<Expr> {
        let left = self.left.resolve(ctx)?;
        let right = self.right.resolve(ctx)?;

        let check_type_mismatch = |ctx: &mut ResolveContext| {
            if left.ty == right.ty {
                true
            } else {
                ctx.push_error(
                    TypeResolveError::TypeMismatchInBinaryOp {
                        op: Located(self.op.0, self.get_location()),
                        left_ty: left.ty,
                        right_ty: right.ty,
                    }
                    .into(),
                );
                false
            }
        };

        use crate::ast::parsing::token::Operator;
        Some(match self.op.0 {
            Operator::Add | Operator::Sub => {
                if !left.ty.is_numeric_ty() {
                    ctx.push_error(
                        TypeResolveError::NonNumericTypeInBinaryOp {
                            op: self.op.clone(),
                            ty: Located(left.ty, self.left.get_location()),
                        }
                        .into(),
                    );
                    return None;
                }
                if !right.ty.is_numeric_ty() {
                    ctx.push_error(
                        TypeResolveError::NonNumericTypeInBinaryOp {
                            op: self.op.clone(),
                            ty: Located(right.ty, self.right.get_location()),
                        }
                        .into(),
                    );
                    return None;
                }

                if !check_type_mismatch(ctx) {
                    return None;
                }

                Expr {
                    ty: left.ty,
                    kind: ExprKind::Binary(BinaryExpr {
                        op: self.op.0,
                        left: Box::new(left),
                        right: Box::new(right),
                    }),
                }
            }
            Operator::And | Operator::Or => {
                if left.ty != PrimType::Bool.into() {
                    ctx.push_error(
                        TypeResolveError::UnexpectedTypeInBinaryOp {
                            op: self.op.clone(),
                            expect_type: PrimType::Bool.into(),
                            actual_type: Located(left.ty, self.left.get_location()),
                        }
                        .into(),
                    );
                }
                if right.ty != PrimType::Bool.into() {
                    ctx.push_error(
                        TypeResolveError::UnexpectedTypeInBinaryOp {
                            op: self.op.clone(),
                            expect_type: PrimType::Bool.into(),
                            actual_type: Located(right.ty, self.left.get_location()),
                        }
                        .into(),
                    );
                }

                Expr {
                    kind: ExprKind::Binary(BinaryExpr {
                        op: self.op.0,
                        left: Box::new(left),
                        right: Box::new(right),
                    }),
                    ty: PrimType::Bool.into(),
                }
            }
            Operator::Equal
            | Operator::NotEqual
            | Operator::Less
            | Operator::LessEqual
            | Operator::Greater
            | Operator::GreaterEqual => {
                if !left.ty.is_ord_ty() {
                    ctx.push_error(
                        TypeResolveError::UnorderedTypeInBinaryOp {
                            op: self.op.clone(),
                            ty: Located(left.ty, self.left.get_location()),
                        }
                        .into(),
                    );
                }
                if !right.ty.is_ord_ty() {
                    ctx.push_error(
                        TypeResolveError::UnorderedTypeInBinaryOp {
                            op: self.op.clone(),
                            ty: Located(right.ty, self.right.get_location()),
                        }
                        .into(),
                    );
                }
                check_type_mismatch(ctx);

                Expr {
                    kind: ExprKind::Binary(BinaryExpr {
                        op: self.op.0,
                        left: Box::new(left),
                        right: Box::new(right),
                    }),
                    ty: PrimType::Bool.into(),
                }
            }
            _ => unimplemented!("binary op '{}'", self.op.0),
        })
    }
}

impl Resolve<Expr> for ast_expr::CallExpr {
    fn resolve(&self, ctx: &mut ResolveContext) -> Option<Expr> {
        let callee = self.callee.resolve(ctx)?;
        let Type::Function(FuncType(func_sym_id)) = callee.ty else {
            ctx.push_error(
                TypeResolveError::CallOnNonFunctionType(Located(callee.ty, self.get_location()))
                    .into(),
            );
            return None;
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
                    func_ty: Located(callee.ty, self.get_location()),
                    expect_count: expect_params.len(),
                    actual_count: self.args.items.len(),
                }
                .into(),
            );
        }

        let mut args = Vec::new();

        for ((i, expect_param), ast_arg) in expect_params.iter().enumerate().zip(&self.args.items) {
            let Some(arg) = ast_arg.resolve(ctx) else {
                continue;
            };

            if expect_param.1 == arg.ty {
                args.push(arg);
                continue;
            }

            ctx.push_error(
                TypeResolveError::ArgumentTypeMismatch {
                    func_ty: Located(callee.ty, self.get_location()),
                    argument_index: i,
                    expect_type: expect_param.1,
                    actual_type: Located(arg.ty, ast_arg.get_location()),
                }
                .into(),
            );
        }

        Some(Expr {
            kind: ExprKind::Call(CallExpr {
                callee: Box::new(callee),
                args,
            }),
            ty: ret_ty,
        })
    }
}
