use crate::{
    ast::{
        self,
        location::{Locatable, Located, LocationRange},
    },
    ast_resolved::{
        errors::{IdentResolveError, TypeResolveError},
        nodes::{
            expr::ExprKind,
            stmt::{AssignStmt, Block, ExprStmt, IfStmt, LetStmt, LoopStmt, ReturnStmt, Stmt},
        },
        ControlFlow, ResolveContext,
    },
    symbol_table::{Symbol, VarSymbol},
    ty::PrimType,
};

use ast::nodes::stmt as ast_stmt;

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
            ast_stmt::Stmt::Block(stmt) => remap!(ctx, stmt, Block),
            ast_stmt::Stmt::Expr(stmt) => remap!(ctx, stmt, Expr),
            ast_stmt::Stmt::Assign(stmt) => remap!(ctx, stmt, Assign),
            ast_stmt::Stmt::Let(stmt) => remap!(ctx, stmt, Let),
            ast_stmt::Stmt::If(stmt) => stmt.resolve(ctx),
            ast_stmt::Stmt::Return(stmt) => remap!(ctx, stmt, Return),
            ast_stmt::Stmt::Loop(stmt) => remap!(ctx, stmt, Loop),
        }
    }
}

impl StmtResolve<Block> for ast_stmt::BlockStmt {
    fn resolve(&self, ctx: &mut ResolveContext) -> (Option<Block>, ControlFlow) {
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
                    let block = Block { table_id, stmts };
                    return (Some(block), ControlFlow::Return);
                }
            };
        }

        ctx.pop_local();
        let block = Block { table_id, stmts };
        (Some(block), ControlFlow::Flow)
    }
}

impl StmtResolve<ExprStmt> for ast_stmt::ExprStmt {
    fn resolve(&self, ctx: &mut ResolveContext) -> (Option<ExprStmt>, ControlFlow) {
        let expr = self.expr.resolve(ctx);
        let stmt = if let Some(expr) = expr {
            if expr.get_kind().is_constant() {
                None
            } else {
                Some(ExprStmt { expr })
            }
        } else {
            None
        };
        (stmt, ControlFlow::Flow)
    }
}

impl StmtResolve<AssignStmt> for ast_stmt::AssignStmt {
    fn resolve(&self, ctx: &mut ResolveContext) -> (Option<AssignStmt>, ControlFlow) {
        let Some(target) = self.target.resolve(ctx) else {
            return (None, ControlFlow::Flow);
        };

        eprintln!("TODO: verify expr is assignable");

        let Some(value) = self.value.resolve(ctx) else {
            return (None, ControlFlow::Flow);
        };

        if target.get_type() != value.get_type() {
            ctx.push_error(
                TypeResolveError::AssignmentTypeMismatch {
                    target_ty: Located(target.get_type(), self.target.get_location()),
                    value_ty: Located(value.get_type(), self.target.get_location()),
                }
                .into(),
            );
        }

        (Some(AssignStmt { target, value }), ControlFlow::Flow)
    }
}

impl StmtResolve<LetStmt> for ast_stmt::LetStmt {
    fn resolve(&self, ctx: &mut ResolveContext) -> (Option<LetStmt>, ControlFlow) {
        let value = self.value.resolve(ctx);

        let value_ty = if let Some(value) = &value {
            if let Some(annotated_ty) = &self.ty {
                if annotated_ty.0 != value.get_type() {
                    ctx.push_error(
                        TypeResolveError::AssignmentTypeMismatch {
                            target_ty: annotated_ty.clone(),
                            value_ty: Located(value.get_type(), self.value.get_location()),
                        }
                        .into(),
                    );
                }
                annotated_ty.0
            } else {
                value.get_type()
            }
        } else if let Some(ty) = &self.ty {
            ty.0
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

impl StmtResolve<Stmt> for ast_stmt::IfStmt {
    fn resolve(&self, ctx: &mut ResolveContext) -> (Option<Stmt>, ControlFlow) {
        let cond = self.cond.resolve(ctx);

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
            if matches!(cond.get_kind(), ExprKind::Bool(false)) {
                return if let Some(else_stmt) = &self.else_stmt {
                    let (body, flow) = else_stmt.body.resolve(ctx);
                    (body.map(|v| Stmt::Block(v)), flow)
                } else {
                    (None, ControlFlow::Flow)
                };
            }
        }

        let (then_block, then_flow) = self.body.resolve(ctx);

        let (else_block, else_flow) = if let Some(else_stmt) = &self.else_stmt {
            else_stmt.body.resolve(ctx)
        } else {
            (None, ControlFlow::Flow)
        };

        let result_flow = if then_flow == ControlFlow::Return && else_flow == ControlFlow::Return {
            ControlFlow::Return
        } else {
            ControlFlow::Flow
        };

        let stmt = if cond.is_some() && then_block.is_some() {
            Some(Stmt::If(IfStmt {
                cond: cond.unwrap(),
                body: then_block.unwrap(),
                else_body: else_block,
            }))
        } else {
            None
        };

        (stmt, result_flow)
    }
}

impl StmtResolve<ReturnStmt> for ast_stmt::ReturnStmt {
    fn resolve(&self, ctx: &mut ResolveContext) -> (Option<ReturnStmt>, ControlFlow) {
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
            let value = expr.resolve(ctx);
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
                    actual_type: Located(
                        val_ty,
                        LocationRange::combine(self.return_tok.1, self.semi_tok.1),
                    ),
                }
                .into(),
            );
        }

        (Some(ReturnStmt { expr: value }), ControlFlow::Return)
    }
}

impl StmtResolve<LoopStmt> for ast_stmt::LoopStmt {
    fn resolve(&self, ctx: &mut ResolveContext) -> (Option<LoopStmt>, ControlFlow) {
        let (body, flow) = self.block.resolve(ctx);
        (body.map(|v| LoopStmt { block: v }), flow)
    }
}
