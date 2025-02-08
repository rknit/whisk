use crate::{
    ast::parsing::token::Operator,
    symbol::{BlockId, NamedId, TypeId},
};

use super::stmt::Stmt;

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub ty: TypeId,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Unit,
    Integer(i64),
    Bool(bool),
    Identifier(IdentExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Call(CallExpr),
    Block(BlockExpr),
    Return(ReturnExpr),
    If(IfExpr),
    Loop(LoopExpr),
}

#[derive(Debug, Clone)]
pub struct IdentExpr {
    pub id: NamedId,
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

#[derive(Debug, Clone)]
pub struct BlockExpr {
    pub block_id: BlockId,
    pub stmts: Vec<Stmt>,
    pub eval_expr: Option<Box<Expr>>,
}

#[derive(Debug, Clone)]
pub struct ReturnExpr {
    pub expr: Option<Box<Expr>>,
}

#[derive(Debug, Clone)]
pub struct IfExpr {
    pub cond: Box<Expr>,
    pub then: BlockExpr,
    pub else_: Option<BlockExpr>,
}

#[derive(Debug, Clone)]
pub struct LoopExpr {
    pub body: BlockExpr,
}

impl From<i64> for ExprKind {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl From<bool> for ExprKind {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
impl From<IdentExpr> for ExprKind {
    fn from(value: IdentExpr) -> Self {
        Self::Identifier(value)
    }
}
impl From<UnaryExpr> for ExprKind {
    fn from(value: UnaryExpr) -> Self {
        Self::Unary(value)
    }
}
impl From<BinaryExpr> for ExprKind {
    fn from(value: BinaryExpr) -> Self {
        Self::Binary(value)
    }
}
impl From<CallExpr> for ExprKind {
    fn from(value: CallExpr) -> Self {
        Self::Call(value)
    }
}
impl From<BlockExpr> for ExprKind {
    fn from(value: BlockExpr) -> Self {
        Self::Block(value)
    }
}
impl From<ReturnExpr> for ExprKind {
    fn from(value: ReturnExpr) -> Self {
        Self::Return(value)
    }
}
impl From<IfExpr> for ExprKind {
    fn from(value: IfExpr) -> Self {
        Self::If(value)
    }
}
impl From<LoopExpr> for ExprKind {
    fn from(value: LoopExpr) -> Self {
        Self::Loop(value)
    }
}
