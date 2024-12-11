use crate::{ast::parsing::token::Operator, ty::Type};

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Integer(i64),
    Bool(bool),
    Identifier(String),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Call(CallExpr),
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub op: Operator,
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub op: Operator,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
}
