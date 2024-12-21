use crate::{
    ast::{
        self,
        location::{Locatable, Located},
    },
    ast_resolved::{
        errors::{IdentResolveError, TypeResolveError, ValueResolveError},
        nodes::expr::{
            BinaryExpr, BlockExpr, CallExpr, Expr, IdentExpr, IfExpr, ReturnExpr, UnaryExpr,
        },
        ControlFlow, ResolveContext,
    },
    symbol_table::Symbol,
    ty::{FuncType, PrimType, Type},
};

use ast::nodes::expr as ast_expr;

use super::stmt::StmtResolve;

#[derive(Debug)]
pub struct ExprFlow(pub Option<Expr>, pub ControlFlow);
impl ExprFlow {
    pub fn flow(expr: impl Into<Expr>) -> Self {
        Self(Some(expr.into()), ControlFlow::Flow)
    }

    pub fn flow_none() -> Self {
        Self(None, ControlFlow::Flow)
    }

    pub fn ret(expr: impl Into<Expr>) -> Self {
        Self(Some(expr.into()), ControlFlow::Return)
    }

    pub fn ret_none() -> Self {
        Self(None, ControlFlow::Return)
    }
}

pub trait ExprResolve {
    fn resolve(&self, ctx: &mut ResolveContext) -> ExprFlow;
}

impl ExprResolve for ast_expr::Expr {
    fn resolve(&self, ctx: &mut ResolveContext) -> ExprFlow {
        match self {
            ast_expr::Expr::Unit(_) => ExprFlow::flow(Expr::Unit),
            ast_expr::Expr::Integer(expr) => ExprFlow::flow(Expr::Integer(expr.0)),
            ast_expr::Expr::Bool(expr) => ExprFlow::flow(Expr::Bool(expr.0)),
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

impl ExprResolve for Located<String> {
    fn resolve(&self, ctx: &mut ResolveContext) -> ExprFlow {
        let Some(symbol) = ctx.get_symbol_by_name_mut(&self.0) else {
            ctx.push_error(IdentResolveError::UnknownIdentifier(self.clone()).into());
            return ExprFlow::flow_none();
        };
        symbol.push_ref(self.1);
        ExprFlow::flow(IdentExpr {
            sym_id: symbol.get_id(),
            ident: self.0.clone(),
            ty: symbol.get_type(),
        })
    }
}

impl ExprResolve for ast_expr::UnaryExpr {
    fn resolve(&self, ctx: &mut ResolveContext) -> ExprFlow {
        let ExprFlow(expr, flow) = self.expr.resolve(ctx);
        if expr.is_none() || flow == ControlFlow::Return {
            return ExprFlow(expr, flow);
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
                    return ExprFlow::flow_none();
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

        ExprFlow::flow(UnaryExpr {
            op: self.op.0,
            expr: Box::new(expr),
            ty,
        })
    }
}

impl ExprResolve for ast_expr::BinaryExpr {
    fn resolve(&self, ctx: &mut ResolveContext) -> ExprFlow {
        let ExprFlow(left, flow) = self.left.resolve(ctx);
        if left.is_none() || flow == ControlFlow::Return {
            return ExprFlow(left, flow);
        }
        let left = left.unwrap();

        let ExprFlow(right, flow) = self.right.resolve(ctx);
        if right.is_none() || flow == ControlFlow::Return {
            return ExprFlow(right, flow);
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
                    return ExprFlow::flow_none();
                }
                if !right.get_type().is_numeric_ty() {
                    ctx.push_error(
                        TypeResolveError::NonNumericTypeInBinaryOp {
                            op: self.op.clone(),
                            ty: Located(right.get_type(), self.right.get_location()),
                        }
                        .into(),
                    );
                    return ExprFlow::flow_none();
                }

                if !check_type_mismatch(ctx) {
                    return ExprFlow::flow_none();
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

        ExprFlow::flow(BinaryExpr {
            op: self.op.0,
            left: Box::new(left),
            right: Box::new(right),
            ty,
        })
    }
}

impl ExprResolve for ast_expr::CallExpr {
    fn resolve(&self, ctx: &mut ResolveContext) -> ExprFlow {
        let ExprFlow(callee, flow) = self.callee.resolve(ctx);
        if callee.is_none() || flow == ControlFlow::Return {
            return ExprFlow(callee, flow);
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
            return ExprFlow::flow_none();
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
            let ExprFlow(arg, flow) = ast_arg.resolve(ctx);
            if flow == ControlFlow::Return {
                return ExprFlow(arg, flow);
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

        ExprFlow::flow(CallExpr {
            callee: Box::new(callee),
            args,
            ty: ret_ty,
        })
    }
}

impl ExprResolve for ast_expr::BlockExpr {
    fn resolve(&self, ctx: &mut ResolveContext) -> ExprFlow {
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
                    return ExprFlow::ret(BlockExpr {
                        table_id,
                        stmts,
                        eval_expr: None,
                        ty: PrimType::Unit.into(),
                    });
                }
            };
        }

        let eval_expr = if let Some(eval_expr) = &self.eval_expr {
            let ExprFlow(expr, flow) = eval_expr.resolve(ctx);
            if expr.is_none() || flow == ControlFlow::Return {
                return ExprFlow(expr, flow);
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
        ExprFlow::flow(BlockExpr {
            table_id,
            stmts,
            eval_expr,
            ty: eval_ty,
        })
    }
}

impl ExprResolve for ast_expr::ReturnExpr {
    fn resolve(&self, ctx: &mut ResolveContext) -> ExprFlow {
        let (func_name, expect_ret_ty) = {
            let func_sym_id = ctx.get_func_symbol_id();
            let Symbol::Function(func) = ctx
                .global_table
                .get_symbol(func_sym_id)
                .expect("already set function symbol id")
            else {
                panic!("symbol is not a function");
            };
            (func.get_name().to_owned(), func.get_return_type())
        };

        let (value, val_ty) = if let Some(expr) = &self.expr {
            let ExprFlow(value, flow) = expr.resolve(ctx);
            if flow == ControlFlow::Return {
                return ExprFlow(value, flow);
            }

            let val_ty = if let Some(value) = &value {
                value.get_type()
            } else {
                expect_ret_ty
            };
            (value, val_ty)
        } else {
            (None, PrimType::Unit.into())
        };

        if expect_ret_ty != val_ty {
            ctx.push_error(
                TypeResolveError::ReturnTypeMismatch {
                    function_name: func_name.to_owned(),
                    expected_type: expect_ret_ty,
                    actual_type: Located(val_ty, self.get_location()),
                }
                .into(),
            );
        }

        let expr = value.map(|v| Box::new(v));
        ExprFlow::ret(ReturnExpr { expr })
    }
}

impl ExprResolve for ast_expr::IfExpr {
    fn resolve(&self, ctx: &mut ResolveContext) -> ExprFlow {
        let ExprFlow(cond, flow) = self.cond.resolve(ctx);
        if flow == ControlFlow::Return {
            return ExprFlow(cond, flow);
        }

        if let Some(cond) = &cond {
            if cond.get_type() != PrimType::Bool.into() {
                ctx.push_error(
                    TypeResolveError::NonBoolInIfCond(Located(
                        cond.get_type(),
                        self.cond.get_location(),
                    ))
                    .into(),
                );
            }

            // dont resolve If stmt when the condition is false
            if matches!(cond, Expr::Bool(false)) {
                return if let Some(else_stmt) = &self.else_expr {
                    else_stmt.body.resolve(ctx)
                } else {
                    ExprFlow::flow_none()
                };
            }
        }

        let ExprFlow(then_block, then_flow) = self.then.resolve(ctx);
        let then_ty = then_block.as_ref().map(|v| v.get_type());

        let ExprFlow(else_block, else_flow) = if let Some(else_stmt) = &self.else_expr {
            else_stmt.body.resolve(ctx)
        } else {
            ExprFlow::flow_none()
        };
        let else_ty = else_block.as_ref().map(|v| v.get_type());

        let result_flow = if then_flow == ControlFlow::Return && else_flow == ControlFlow::Return {
            ControlFlow::Return
        } else {
            ControlFlow::Flow
        };

        let result_ty = if let (Some(then_ty), Some(else_ty)) = (then_ty, else_ty) {
            if then_ty != else_ty {
                ctx.push_error(
                    TypeResolveError::BlockBranchTypeMismatch {
                        branch: Located(then_ty, self.then.get_location()),
                        other: Located(else_ty, self.else_expr.as_ref().unwrap().get_location()),
                    }
                    .into(),
                );
                None
            } else {
                Some(then_ty)
            }
        } else if let Some(ty) = then_ty.or(else_ty) {
            Some(ty)
        } else {
            None
        };

        if let (Some(cond), Some(then_block)) = (cond, then_block) {
            let Expr::Block(then) = then_block else {
                unreachable!()
            };
            let else_ = else_block.map(|v| {
                let Expr::Block(else_) = v else {
                    unreachable!()
                };
                else_
            });
            ExprFlow(
                Some(
                    IfExpr {
                        cond: Box::new(cond),
                        then,
                        else_,
                        ty: result_ty.unwrap(),
                    }
                    .into(),
                ),
                result_flow,
            )
        } else {
            ExprFlow(None, result_flow)
        }
    }
}
