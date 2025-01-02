use crate::ast::{
    location::{Locatable, Located, Span},
    parsing::token::{Delimiter, Keyword, Operator},
};

use super::{punctuate::Punctuated, stmt::Stmt};

#[derive(Debug, Clone)]
pub enum Expr {
    Unit(Span),
    Integer(Located<i64>),
    Bool(Located<bool>),
    Identifier(Located<String>),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Grouped(GroupedExpr),
    Call(CallExpr),
    Block(BlockExpr),
    Return(ReturnExpr),
    If(IfExpr),
    Loop(LoopExpr),
}
impl Expr {
    pub fn is_block(&self) -> bool {
        matches!(self, Self::Block(_) | Self::If(_) | Self::Loop(_))
    }

    pub fn has_eval_expr(&self) -> bool {
        match self {
            Self::Block(v) => v.eval_expr.is_some(),
            Self::If(v) => {
                v.then.eval_expr.is_some()
                    && v.else_expr
                        .as_ref()
                        .map(|v| v.body.eval_expr.is_some())
                        .unwrap_or(false)
            }
            Self::Loop(v) => v.body.eval_expr.is_some(),
            _ => false,
        }
    }
}
impl Locatable for Expr {
    fn get_location(&self) -> Span {
        match self {
            Expr::Unit(loc) => *loc,
            Expr::Integer(located) => located.1,
            Expr::Bool(located) => located.1,
            Expr::Identifier(located) => located.1,
            Expr::Unary(unary_expr) => unary_expr.get_location(),
            Expr::Binary(binary_expr) => binary_expr.get_location(),
            Expr::Grouped(grouped_expr) => grouped_expr.get_location(),
            Expr::Call(call_expr) => call_expr.get_location(),
            Expr::Block(expr) => expr.get_location(),
            Expr::Return(expr) => expr.get_location(),
            Expr::If(expr) => expr.get_location(),
            Expr::Loop(expr) => expr.get_location(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub op: Located<Operator>,
    pub expr: Box<Expr>,
}
impl Locatable for UnaryExpr {
    fn get_location(&self) -> Span {
        Span::combine(self.op.1, self.expr.get_location())
    }
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub op: Located<Operator>,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}
impl Locatable for BinaryExpr {
    fn get_location(&self) -> Span {
        Span::combine(self.left.get_location(), self.right.get_location())
    }
}

#[derive(Debug, Clone)]
pub struct GroupedExpr {
    pub paren_open_tok: Located<Delimiter>,
    pub expr: Box<Expr>,
    pub paren_close_tok: Located<Delimiter>,
}
impl Locatable for GroupedExpr {
    fn get_location(&self) -> Span {
        Span::combine(self.paren_open_tok.1, self.paren_close_tok.1)
    }
}

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub paren_open_tok: Located<Delimiter>,
    pub args: Punctuated<Expr>,
    pub paren_close_tok: Located<Delimiter>,
}
impl Locatable for CallExpr {
    fn get_location(&self) -> Span {
        Span::combine(self.callee.get_location(), self.paren_close_tok.1)
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
    fn get_location(&self) -> Span {
        Span::combine(self.brace_open_tok.1, self.brace_close_tok.1)
    }
}

#[derive(Debug, Clone)]
pub struct ReturnExpr {
    pub return_tok: Located<Keyword>,
    pub expr: Option<Box<Expr>>,
}
impl Locatable for ReturnExpr {
    fn get_location(&self) -> Span {
        let end_loc = if let Some(expr) = &self.expr {
            expr.get_location()
        } else {
            self.return_tok.1
        };
        Span::combine(self.return_tok.1, end_loc)
    }
}

#[derive(Debug, Clone)]
pub struct IfExpr {
    pub if_tok: Located<Keyword>,
    pub cond: Box<Expr>,
    pub then: BlockExpr,
    pub else_expr: Option<ElseExpr>,
}
impl Locatable for IfExpr {
    fn get_location(&self) -> Span {
        if let Some(else_expr) = &self.else_expr {
            Span::combine(self.if_tok.1, else_expr.get_location())
        } else {
            Span::combine(self.if_tok.1, self.then.get_location())
        }
    }
}

#[derive(Debug, Clone)]
pub struct ElseExpr {
    pub else_tok: Located<Keyword>,
    pub body: BlockExpr,
}
impl Locatable for ElseExpr {
    fn get_location(&self) -> Span {
        Span::combine(self.else_tok.1, self.body.get_location())
    }
}

#[derive(Debug, Clone)]
pub struct LoopExpr {
    pub loop_tok: Located<Keyword>,
    pub body: BlockExpr,
}
impl Locatable for LoopExpr {
    fn get_location(&self) -> Span {
        Span::combine(self.loop_tok.1, self.body.get_location())
    }
}
