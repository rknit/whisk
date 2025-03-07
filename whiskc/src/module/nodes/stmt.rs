use crate::symbol::VarId;

use super::expr::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(ExprStmt),
    Let(LetStmt),
}

#[derive(Debug, Clone)]
pub struct ExprStmt {
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub struct LetStmt {
    pub var_id: VarId,
    pub value: Expr,
}
