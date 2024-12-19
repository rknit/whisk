use crate::{
    ast::{
        location::{Locatable, Located, LocationRange},
        parsing::token::{Delimiter, Keyword, Operator},
    },
    ty::Type,
};

use super::{punctuate::Puntuated, stmt::Stmt};

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(Located<i64>),
    Bool(Located<bool>),
    Identifier(Located<String>),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Grouped(GroupedExpr),
    Call(CallExpr),
    Array(ArrayExpr),
    ArrayIndex(ArrayIndexExpr),
    Cast(CastExpr),
    Block(BlockExpr),
}
impl Locatable for Expr {
    fn get_location(&self) -> LocationRange {
        match self {
            Expr::Integer(located) => located.1,
            Expr::Bool(located) => located.1,
            Expr::Identifier(located) => located.1,
            Expr::Unary(unary_expr) => unary_expr.get_location(),
            Expr::Binary(binary_expr) => binary_expr.get_location(),
            Expr::Grouped(grouped_expr) => grouped_expr.get_location(),
            Expr::Call(call_expr) => call_expr.get_location(),
            Expr::Array(array_expr) => array_expr.get_location(),
            Expr::ArrayIndex(array_index_expr) => array_index_expr.get_location(),
            Expr::Cast(cast_expr) => cast_expr.get_location(),
            Expr::Block(expr) => expr.get_location(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub op: Located<Operator>,
    pub expr: Box<Expr>,
}
impl Locatable for UnaryExpr {
    fn get_location(&self) -> LocationRange {
        LocationRange::combine(self.op.1, self.expr.get_location())
    }
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub op: Located<Operator>,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}
impl Locatable for BinaryExpr {
    fn get_location(&self) -> LocationRange {
        LocationRange::combine(self.left.get_location(), self.right.get_location())
    }
}

#[derive(Debug, Clone)]
pub struct GroupedExpr {
    pub paren_open_tok: Located<Delimiter>,
    pub expr: Box<Expr>,
    pub paren_close_tok: Located<Delimiter>,
}
impl Locatable for GroupedExpr {
    fn get_location(&self) -> LocationRange {
        LocationRange::combine(self.paren_open_tok.1, self.paren_close_tok.1)
    }
}

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub paren_open_tok: Located<Delimiter>,
    pub args: Puntuated<Expr>,
    pub paren_close_tok: Located<Delimiter>,
}
impl Locatable for CallExpr {
    fn get_location(&self) -> LocationRange {
        LocationRange::combine(self.callee.get_location(), self.paren_close_tok.1)
    }
}

#[derive(Debug, Clone)]
pub struct ArrayExpr {
    pub bracket_open_tok: Located<Delimiter>,
    pub elements: Puntuated<Expr>,
    pub bracket_close_tok: Located<Delimiter>,
}
impl Locatable for ArrayExpr {
    fn get_location(&self) -> LocationRange {
        LocationRange::combine(self.bracket_open_tok.1, self.bracket_close_tok.1)
    }
}

#[derive(Debug, Clone)]
pub struct ArrayIndexExpr {
    pub expr: Box<Expr>,
    pub bracket_open_tok: Located<Delimiter>,
    pub index: Box<Expr>,
    pub bracket_close_tok: Located<Delimiter>,
}
impl Locatable for ArrayIndexExpr {
    fn get_location(&self) -> LocationRange {
        LocationRange::combine(self.expr.get_location(), self.bracket_close_tok.1)
    }
}

#[derive(Debug, Clone)]
pub struct CastExpr {
    pub expr: Box<Expr>,
    pub as_tok: Located<Keyword>,
    pub ty: Located<Type>,
}
impl Locatable for CastExpr {
    fn get_location(&self) -> LocationRange {
        LocationRange::combine(self.expr.get_location(), self.ty.1)
    }
}

#[derive(Debug, Clone)]
pub struct BlockExpr {
    pub brace_open_tok: Located<Delimiter>,
    pub stmts: Vec<Stmt>,
    pub eval_expr: Option<Box<Expr>>,
    pub brace_close_tok: Located<Delimiter>,
}
impl Locatable for BlockExpr {
    fn get_location(&self) -> LocationRange {
        LocationRange::combine(self.brace_open_tok.1, self.brace_close_tok.1)
    }
}
