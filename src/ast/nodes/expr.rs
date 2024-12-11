use core::fmt;

use crate::{
    ast::{
        location::{Locatable, Located, LocationRange},
        parsing::token::{Delimiter, Keyword, Operator},
    },
    ty::Type,
};

use super::punctuate::Puntuated;

#[derive(Clone)]
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
        }
    }
}
impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Integer(v) => write!(f, "{:?}", v),
            Expr::Bool(v) => write!(f, "{:?}", v),
            Expr::Identifier(v) => write!(f, "{:?}", v),
            Expr::Unary(v) => write!(f, "{:?}", v),
            Expr::Binary(v) => write!(f, "{:?}", v),
            Expr::Grouped(v) => write!(f, "{:?}", v),
            Expr::Call(v) => write!(f, "{:?}", v),
            Expr::Array(v) => write!(f, "{:?}", v),
            Expr::ArrayIndex(v) => write!(f, "{:?}", v),
            Expr::Cast(v) => write!(f, "{:?}", v),
        }
    }
}

#[derive(Clone)]
pub struct UnaryExpr {
    pub op: Located<Operator>,
    pub expr: Box<Expr>,
}
impl fmt::Debug for UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{:?} {:?}>", self.op, self.expr)
    }
}
impl Locatable for UnaryExpr {
    fn get_location(&self) -> LocationRange {
        LocationRange::combine(self.op.1, self.expr.get_location())
    }
}

#[derive(Clone)]
pub struct BinaryExpr {
    pub op: Located<Operator>,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}
impl fmt::Debug for BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{:?} {:?} {:?}>", self.op, self.left, self.right)
    }
}
impl Locatable for BinaryExpr {
    fn get_location(&self) -> LocationRange {
        LocationRange::combine(self.left.get_location(), self.right.get_location())
    }
}

#[derive(Clone)]
pub struct GroupedExpr {
    pub paren_open_tok: Located<Delimiter>,
    pub expr: Box<Expr>,
    pub paren_close_tok: Located<Delimiter>,
}
impl fmt::Debug for GroupedExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?})", self.expr)
    }
}
impl Locatable for GroupedExpr {
    fn get_location(&self) -> LocationRange {
        LocationRange::combine(self.paren_open_tok.1, self.paren_close_tok.1)
    }
}

#[derive(Clone)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub paren_open_tok: Located<Delimiter>,
    pub args: Puntuated<Expr>,
    pub paren_close_tok: Located<Delimiter>,
}
impl fmt::Debug for CallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}({})",
            self.callee,
            self.args
                .items
                .iter()
                .map(|v| format!("{:?}", v))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
impl Locatable for CallExpr {
    fn get_location(&self) -> LocationRange {
        LocationRange::combine(self.callee.get_location(), self.paren_close_tok.1)
    }
}

#[derive(Clone)]
pub struct ArrayExpr {
    pub bracket_open_tok: Located<Delimiter>,
    pub elements: Puntuated<Expr>,
    pub bracket_close_tok: Located<Delimiter>,
}
impl fmt::Debug for ArrayExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.elements
                .items
                .iter()
                .map(|v| format!("{:?}", v))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
impl Locatable for ArrayExpr {
    fn get_location(&self) -> LocationRange {
        LocationRange::combine(self.bracket_open_tok.1, self.bracket_close_tok.1)
    }
}

#[derive(Clone)]
pub struct ArrayIndexExpr {
    pub expr: Box<Expr>,
    pub bracket_open_tok: Located<Delimiter>,
    pub index: Box<Expr>,
    pub bracket_close_tok: Located<Delimiter>,
}
impl fmt::Debug for ArrayIndexExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}[{:?}]", self.expr, self.index)
    }
}
impl Locatable for ArrayIndexExpr {
    fn get_location(&self) -> LocationRange {
        LocationRange::combine(self.expr.get_location(), self.bracket_close_tok.1)
    }
}

#[derive(Clone)]
pub struct CastExpr {
    pub expr: Box<Expr>,
    pub as_tok: Located<Keyword>,
    pub ty: Located<Type>,
}
impl fmt::Debug for CastExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{:?} {:?} {:?}>", self.as_tok, self.ty, self.expr)
    }
}
impl Locatable for CastExpr {
    fn get_location(&self) -> LocationRange {
        LocationRange::combine(self.expr.get_location(), self.ty.1)
    }
}
