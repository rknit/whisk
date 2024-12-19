use core::fmt;

use crate::{
    ast::{
        location::Located,
        parsing::token::{Delimiter, Keyword, Operator},
    },
    ty::Type,
};

use super::expr::Expr;

#[derive(Clone)]
pub enum Stmt {
    Block(Block),
    Expr(ExprStmt),
    Assign(AssignStmt),
    Let(LetStmt),
    If(IfStmt),
    Return(ReturnStmt),
    Loop(LoopStmt),
}
impl fmt::Debug for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Block(v) => write!(f, "{:#?}", v),
            Stmt::Expr(v) => write!(f, "{:#?}", v),
            Stmt::Assign(v) => write!(f, "{:#?}", v),
            Stmt::Let(v) => write!(f, "{:#?}", v),
            Stmt::If(v) => write!(f, "{:#?}", v),
            Stmt::Return(v) => write!(f, "{:#?}", v),
            Stmt::Loop(v) => write!(f, "{:#?}", v),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub brace_open_tok: Located<Delimiter>,
    pub stmts: Vec<Stmt>,
    pub brace_close_tok: Located<Delimiter>,
}

#[derive(Debug, Clone)]
pub struct ExprStmt {
    pub expr: Expr,
    pub semi_tok: Located<Delimiter>,
}

#[derive(Debug, Clone)]
pub struct AssignStmt {
    pub target: Expr,
    pub assign_tok: Located<Operator>,
    pub value: Expr,
    pub semi_tok: Located<Delimiter>,
}

#[derive(Debug, Clone)]
pub struct LetStmt {
    pub let_tok: Located<Keyword>,
    pub name: Located<String>,
    pub ty: Option<Located<Type>>,
    pub assign_tok: Located<Operator>,
    pub value: Expr,
    pub semi_tok: Located<Delimiter>,
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub if_tok: Located<Keyword>,
    pub cond: Expr,
    pub body: Block,
    pub else_stmt: Option<ElseStmt>,
}

#[derive(Debug, Clone)]
pub struct ElseStmt {
    pub else_tok: Located<Keyword>,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct ReturnStmt {
    pub return_tok: Located<Keyword>,
    pub expr: Option<Expr>,
    pub semi_tok: Located<Delimiter>,
}

#[derive(Debug, Clone)]
pub struct LoopStmt {
    pub loop_tok: Located<Keyword>,
    pub block: Block,
}
