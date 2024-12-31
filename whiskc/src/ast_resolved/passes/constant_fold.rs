use crate::{
    ast::parsing::token::Operator,
    ast_resolved::{
        nodes::expr::{BinaryExpr, Expr, UnaryExpr},
        visit::{self, VisitMut},
        ResolvedAST,
    },
};

#[derive(Debug)]
struct FoldVisitor {}

pub fn constant_fold(ast: &mut ResolvedAST) {
    let mut v = FoldVisitor {};
    v.visit_ast_mut(ast);
}

impl VisitMut for FoldVisitor {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        visit::visit_expr_mut(self, node);
        match node {
            // TODO: Expr::Identifier(_) => todo!(),
            Expr::Unary(_) => constant_fold_unary(node),
            Expr::Binary(_) => constant_fold_binary(node),
            _ => (),
        }
    }
}

fn constant_fold_unary(expr: &mut Expr) {
    let Expr::Unary(UnaryExpr {
        op, expr: value, ..
    }) = expr
    else {
        return;
    };
    match op {
        Operator::Sub => {
            let Expr::Integer(v) = value.as_ref() else {
                return;
            };
            *expr = Expr::Integer(-v);
        }
        Operator::Not => {
            let Expr::Bool(v) = value.as_ref() else {
                return;
            };
            *expr = Expr::Bool(!v);
        }
        _ => (),
    }
}

macro_rules! bin_op {
    ($expr:expr, $left:expr, $right:expr, $op:tt, $param_kind:ident, $result_kind:ident) => {{
        let (Expr::$param_kind(left), Expr::$param_kind(right)) = ($left.as_ref(), $right.as_ref()) else {
            return;
        };
        *$expr = Expr::$result_kind(*left $op *right);
    }};
}

fn constant_fold_binary(expr: &mut Expr) {
    let Expr::Binary(BinaryExpr {
        op, left, right, ..
    }) = expr
    else {
        return;
    };
    match op {
        Operator::Add => bin_op!(expr, left, right, +, Integer, Integer),
        Operator::Sub => bin_op!(expr, left, right, -, Integer, Integer),
        Operator::And => bin_op!(expr, left, right, &&, Bool, Bool),
        Operator::Or => bin_op!(expr, left, right, ||, Bool, Bool),
        Operator::Equal => bin_op!(expr, left, right, ==, Integer, Bool),
        Operator::NotEqual => bin_op!(expr, left, right, !=, Integer, Bool),
        Operator::Less => bin_op!(expr, left, right, <, Integer, Bool),
        Operator::LessEqual => bin_op!(expr, left, right, <=, Integer, Bool),
        Operator::Greater => bin_op!(expr, left, right, >, Integer, Bool),
        Operator::GreaterEqual => bin_op!(expr, left, right, >=, Integer, Bool),
        _ => (),
    }
}
