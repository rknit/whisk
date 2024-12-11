use crate::{ast::location::Located, symbol_table::SymbolID, ty::Type};

use super::expr::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Block),
    Expr(ExprStmt),
    Assign(AssignStmt),
    Let(LetStmt),
    If(IfStmt),
    Return(ReturnStmt),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub table_id: SymbolID,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct ExprStmt {
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub struct AssignStmt {
    pub target: Expr,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct LetStmt {
    pub sym_id: SymbolID,
    pub name: Located<String>,
    pub ty: Type,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub cond: Expr,
    pub body: Block,
    pub else_body: Option<Block>,
}

#[derive(Debug, Clone)]
pub struct ReturnStmt {
    pub expr: Option<Expr>,
}
