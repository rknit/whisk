use crate::{ast::parsing::token::Operator, symbol_table::SymbolID};

use super::{stmt::Stmt, ty::Type};

#[derive(Debug, Clone)]
pub enum Expr {
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
impl Expr {
    pub fn is_constant(&self) -> bool {
        matches!(self, Self::Unit | Self::Integer(_) | Self::Bool(_))
    }

    pub fn get_type(&self) -> Type {
        match self {
            Expr::Unit => Type::Unit,
            Expr::Integer(_) => Type::Int,
            Expr::Bool(_) => Type::Bool,
            Expr::Identifier(expr) => expr.ty,
            Expr::Unary(expr) => expr.ty,
            Expr::Binary(expr) => expr.ty,
            Expr::Call(expr) => expr.ty,
            Expr::Block(expr) => expr.ty,
            Expr::Return(_) => Type::Never,
            Expr::If(expr) => expr.ty,
            Expr::Loop(expr) => expr.ty,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IdentExpr {
    pub sym_id: SymbolID,
    pub ident: String,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub op: Operator,
    pub expr: Box<Expr>,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub op: Operator,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct BlockExpr {
    pub table_id: SymbolID,
    pub stmts: Vec<Stmt>,
    pub eval_expr: Option<Box<Expr>>,
    pub ty: Type,
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
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct LoopExpr {
    pub body: BlockExpr,
    pub ty: Type,
}

impl From<i64> for Expr {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl From<bool> for Expr {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
impl From<IdentExpr> for Expr {
    fn from(value: IdentExpr) -> Self {
        Self::Identifier(value)
    }
}
impl From<UnaryExpr> for Expr {
    fn from(value: UnaryExpr) -> Self {
        Self::Unary(value)
    }
}
impl From<BinaryExpr> for Expr {
    fn from(value: BinaryExpr) -> Self {
        Self::Binary(value)
    }
}
impl From<CallExpr> for Expr {
    fn from(value: CallExpr) -> Self {
        Self::Call(value)
    }
}
impl From<BlockExpr> for Expr {
    fn from(value: BlockExpr) -> Self {
        Self::Block(value)
    }
}
impl From<ReturnExpr> for Expr {
    fn from(value: ReturnExpr) -> Self {
        Self::Return(value)
    }
}
impl From<IfExpr> for Expr {
    fn from(value: IfExpr) -> Self {
        Self::If(value)
    }
}
impl From<LoopExpr> for Expr {
    fn from(value: LoopExpr) -> Self {
        Self::Loop(value)
    }
}
