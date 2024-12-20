use crate::{
    ast::{
        location::Located,
        parsing::token::{Delimiter, Keyword, Operator},
    },
    ty::Type,
};

use super::expr::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(ExprStmt),
    Let(LetStmt),
}

#[derive(Debug, Clone)]
pub struct ExprStmt {
    pub expr: Expr,
    pub semi_tok: Option<Located<Delimiter>>,
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
