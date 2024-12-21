use crate::ast::parsing::token::Operator;

use super::nodes::expr::{BinaryExpr, Expr, UnaryExpr};

pub trait EvalConstant {
    fn eval_constant(&self) -> Option<Expr>;
}

impl EvalConstant for Expr {
    fn eval_constant(&self) -> Option<Expr> {
        match self {
            Expr::Integer(_) | Expr::Bool(_) | Expr::Identifier(_) | Expr::Call(_) => None,
            Expr::Unary(expr) => expr.eval_constant(),
            Expr::Binary(expr) => expr.eval_constant(),
            _ => todo!(),
        }
    }
}

impl EvalConstant for UnaryExpr {
    fn eval_constant(&self) -> Option<Expr> {
        match self.op {
            Operator::Sub => match *self.expr {
                Expr::Integer(v) => Some(Expr::Integer(-v)),
                _ => None,
            },
            Operator::Not => match *self.expr {
                Expr::Bool(v) => Some(Expr::Bool(!v)),
                _ => None,
            },
            _ => unimplemented!("EvalConstant unary"),
        }
    }
}

impl EvalConstant for BinaryExpr {
    fn eval_constant(&self) -> Option<Expr> {
        let get_ints = || match (self.left.as_ref(), self.right.as_ref()) {
            (Expr::Integer(lhs), Expr::Integer(rhs)) => Some((lhs, rhs)),
            _ => None,
        };
        let get_bools = || match (self.left.as_ref(), self.right.as_ref()) {
            (Expr::Bool(lhs), Expr::Bool(rhs)) => Some((lhs, rhs)),
            _ => None,
        };
        let get_bools_laxed = || {
            let lhs = if let Expr::Bool(lhs) = *self.left {
                Some(lhs)
            } else {
                None
            };
            let rhs = if let Expr::Bool(rhs) = *self.right {
                Some(rhs)
            } else {
                None
            };
            (lhs, rhs)
        };
        Some(match self.op {
            Operator::Add => {
                let (lhs, rhs) = get_ints()?;
                Expr::Integer(lhs + rhs)
            }
            Operator::Sub => {
                let (lhs, rhs) = get_ints()?;
                Expr::Integer(lhs - rhs)
            }
            Operator::And => {
                let (lhs, rhs) = get_bools_laxed();
                if let Some(false) = lhs {
                    false.into()
                } else if let Some(false) = rhs {
                    false.into()
                } else if let (Some(lhs), Some(rhs)) = (lhs, rhs) {
                    (lhs && rhs).into()
                } else {
                    return None;
                }
            }
            Operator::Or => {
                let (lhs, rhs) = get_bools_laxed();
                if let Some(true) = lhs {
                    true.into()
                } else if let Some(true) = rhs {
                    true.into()
                } else if let (Some(lhs), Some(rhs)) = (lhs, rhs) {
                    (lhs || rhs).into()
                } else {
                    return None;
                }
            }
            Operator::Equal => Expr::Bool(if let Some((lhs, rhs)) = get_ints() {
                lhs == rhs
            } else if let Some((lhs, rhs)) = get_bools() {
                lhs == rhs
            } else {
                return None;
            }),
            Operator::NotEqual => Expr::Bool(if let Some((lhs, rhs)) = get_ints() {
                lhs != rhs
            } else if let Some((lhs, rhs)) = get_bools() {
                lhs != rhs
            } else {
                return None;
            }),
            Operator::Less => {
                let (lhs, rhs) = get_ints()?;
                Expr::Bool(lhs < rhs)
            }
            Operator::LessEqual => {
                let (lhs, rhs) = get_ints()?;
                Expr::Bool(lhs <= rhs)
            }
            Operator::Greater => {
                let (lhs, rhs) = get_ints()?;
                Expr::Bool(lhs > rhs)
            }
            Operator::GreaterEqual => {
                let (lhs, rhs) = get_ints()?;
                Expr::Bool(lhs >= rhs)
            }
            _ => unimplemented!("EvalConstant binary"),
        })
    }
}
