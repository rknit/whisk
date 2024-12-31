use std::collections::HashMap;

use crate::{
    ast::parsing::token::Operator,
    ast_resolved::{
        nodes::{
            expr::{BinaryExpr, Expr, UnaryExpr},
            stmt::LetStmt,
        },
        visit::{self, VisitMut},
        ResolvedAST,
    },
    symbol_table::SymbolID,
};

#[derive(Debug, Default)]
struct FoldVisitor {
    values: HashMap<SymbolID, Expr>,
}
impl FoldVisitor {
    pub fn set_value(&mut self, sym_id: SymbolID, value: Expr) {
        if let Some(v) = self.values.get_mut(&sym_id) {
            *v = value;
        } else {
            self.values.insert(sym_id, value);
        }
    }

    pub fn reset_value(&mut self, sym_id: SymbolID) {
        self.values.remove(&sym_id);
    }

    pub fn get_value(&self, sym_id: SymbolID) -> Option<&Expr> {
        self.values.get(&sym_id)
    }
}

pub fn constant_fold(ast: &mut ResolvedAST) {
    let mut v = FoldVisitor::default();
    v.visit_ast_mut(ast);
}

impl VisitMut for FoldVisitor {
    fn visit_let_stmt_mut(&mut self, node: &mut LetStmt) {
        visit::visit_let_stmt_mut(self, node);
        if !node.value.is_constant() {
            return;
        }
        self.set_value(node.sym_id, node.value.clone());
    }

    fn visit_expr_mut(&mut self, node: &mut Expr) {
        visit::visit_expr_mut(self, node);
        match node {
            Expr::Identifier(_) => constant_fold_ident(self, node),
            Expr::Unary(_) => constant_fold_unary(node),
            Expr::Binary(_) => constant_fold_binary(self, node),
            _ => (),
        }
    }

    fn visit_binary_expr_mut(&mut self, node: &mut BinaryExpr) {
        visit::visit_expr_mut(self, &mut node.right);
    }
}

fn constant_fold_ident(v: &FoldVisitor, ident_expr: &mut Expr) {
    let Expr::Identifier(ident) = ident_expr else {
        return;
    };
    let Some(expr) = v.get_value(ident.sym_id) else {
        return;
    };
    *ident_expr = expr.clone();
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

fn constant_fold_binary(v: &mut FoldVisitor, expr: &mut Expr) {
    let Expr::Binary(BinaryExpr {
        op, left, right, ..
    }) = expr
    else {
        return;
    };
    match op {
        Operator::Assign => {
            let Expr::Identifier(left) = left.as_ref() else {
                return;
            };
            if !right.is_constant() {
                v.reset_value(left.sym_id);
                return;
            }
            v.set_value(left.sym_id, right.as_ref().clone());
        }
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
