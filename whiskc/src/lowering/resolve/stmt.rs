use crate::{
    ast::location::{Locatable, Located},
    lowering::{
        errors::{IdentResolveError, TypeResolveError},
        nodes::stmt::{ExprStmt, LetStmt, Stmt},
    },
    symbol_table::VarSymbol,
};

use crate::ast::nodes::stmt as ast_stmt;

use super::{
    expr::{ExprFlow, ExprResolve},
    ControlFlow, Resolve, ResolveContext,
};

pub trait StmtResolve<T> {
    fn resolve(&self, ctx: &mut ResolveContext) -> (Option<T>, ControlFlow);
}

macro_rules! remap {
    ($ctx:expr, $v:expr, $t:ident) => {{
        let (v, flow) = $v.resolve($ctx);
        (v.map(|v| Stmt::$t(v)), flow)
    }};
}

impl StmtResolve<Stmt> for ast_stmt::Stmt {
    fn resolve(&self, ctx: &mut ResolveContext) -> (Option<Stmt>, ControlFlow) {
        match self {
            ast_stmt::Stmt::Expr(stmt) => remap!(ctx, stmt, Expr),
            ast_stmt::Stmt::Let(stmt) => remap!(ctx, stmt, Let),
        }
    }
}

impl StmtResolve<ExprStmt> for ast_stmt::ExprStmt {
    fn resolve(&self, ctx: &mut ResolveContext) -> (Option<ExprStmt>, ControlFlow) {
        let ExprFlow(expr, flow) = self.expr.resolve(ctx);

        let stmt = if let Some(expr) = expr {
            // discard constant values
            if expr.is_constant() {
                None
            } else {
                Some(ExprStmt { expr })
            }
        } else {
            None
        };

        (stmt, flow)
    }
}

impl StmtResolve<LetStmt> for ast_stmt::LetStmt {
    fn resolve(&self, ctx: &mut ResolveContext) -> (Option<LetStmt>, ControlFlow) {
        let ExprFlow(value, flow) = self.value.resolve(ctx);
        if flow == ControlFlow::Return {
            return (None, flow);
        }

        let annotated_ty = self.ty.as_ref().and_then(|v| v.resolve(ctx));

        let value_ty = if let Some(value) = &value {
            if let Some(annotated_ty) = annotated_ty {
                if annotated_ty != value.get_type() {
                    ctx.push_error(
                        TypeResolveError::AssignmentTypeMismatch {
                            target_ty: Located(
                                annotated_ty,
                                self.ty.as_ref().unwrap().get_location(),
                            ),
                            value_ty: Located(value.get_type(), self.value.get_location()),
                        }
                        .into(),
                    );
                }
                annotated_ty
            } else {
                value.get_type()
            }
        } else if let Some(ty) = annotated_ty {
            ty
        } else {
            return (None, ControlFlow::Flow);
        };

        let var_sym = VarSymbol::new(self.name.to_owned(), value_ty);
        let Some(sym_id) = ctx.new_symbol(&self.name.0, var_sym.into()) else {
            let first_origin = {
                let first_sym = ctx
                    .get_current_table_mut()
                    .get_symbol_by_name(&self.name.0)
                    .unwrap();
                (first_sym.get_type(), first_sym.get_origin())
            };
            ctx.push_error(
                IdentResolveError::VarNameAlreadyUsed {
                    ident: self.name.0.clone(),
                    first_origin,
                    dup_origin: (value_ty, self.name.1),
                }
                .into(),
            );
            return (None, ControlFlow::Flow);
        };

        if let Some(value) = value {
            (
                Some(LetStmt {
                    sym_id,
                    name: self.name.clone(),
                    ty: value_ty,
                    value,
                }),
                ControlFlow::Flow,
            )
        } else {
            (None, ControlFlow::Flow)
        }
    }
}
