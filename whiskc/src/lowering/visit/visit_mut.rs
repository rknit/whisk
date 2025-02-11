use crate::lowering::nodes::expr::ExprKind;

use super::super::{
    nodes::{
        expr::{
            BinaryExpr, BlockExpr, CallExpr, Expr, IfExpr, LoopExpr, ReturnExpr, UnaryExpr,
            VarIdentExpr,
        },
        func::{ExternFunction, Function},
        item::Item,
        stmt::{ExprStmt, LetStmt, Stmt},
    },
    Module,
};

pub trait VisitMut: Sized {
    fn visit_module_mut(&mut self, node: &mut Module) {
        visit_module_mut(self, node);
    }

    fn visit_binary_expr_mut(&mut self, node: &mut BinaryExpr) {
        visit_binary_expr_mut(self, node);
    }

    fn visit_block_expr_mut(&mut self, node: &mut BlockExpr) {
        visit_block_expr_mut(self, node);
    }

    fn visit_bool_expr_mut(&mut self, _value: &mut bool) {
        /* terminal */
    }

    fn visit_call_expr_mut(&mut self, node: &mut CallExpr) {
        visit_call_expr_mut(self, node);
    }

    fn visit_expr_mut(&mut self, node: &mut Expr) {
        visit_expr_mut(self, node);
    }

    fn visit_expr_stmt_mut(&mut self, node: &mut ExprStmt) {
        visit_expr_stmt_mut(self, node);
    }

    fn visit_extern_func_mut(&mut self, _node: &mut ExternFunction) {
        /* terminal */
    }

    fn visit_func_mut(&mut self, node: &mut Function) {
        visit_func_mut(self, node);
    }

    fn visit_var_ident_expr_mut(&mut self, _node: &mut VarIdentExpr) {
        /* terminal */
    }

    fn visit_if_expr_mut(&mut self, node: &mut IfExpr) {
        visit_if_expr_mut(self, node);
    }

    fn visit_int_expr_mut(&mut self, _value: &mut i64) {
        /* terminal */
    }

    fn visit_item_mut(&mut self, node: &mut Item) {
        visit_item_mut(self, node);
    }

    fn visit_let_stmt_mut(&mut self, node: &mut LetStmt) {
        visit_let_stmt_mut(self, node);
    }

    fn visit_loop_expr_mut(&mut self, node: &mut LoopExpr) {
        visit_loop_expr_mut(self, node);
    }

    fn visit_return_expr_mut(&mut self, node: &mut ReturnExpr) {
        visit_return_expr_mut(self, node);
    }

    fn visit_stmt_mut(&mut self, node: &mut Stmt) {
        visit_stmt_mut(self, node);
    }

    fn visit_unary_expr_mut(&mut self, node: &mut UnaryExpr) {
        visit_unary_expr_mut(self, node);
    }

    fn visit_unit_expr_mut(&mut self) {
        /* terminal */
    }
}

pub fn visit_module_mut(v: &mut impl VisitMut, node: &mut Module) {
    for item in &mut node.items {
        v.visit_item_mut(item);
    }
}

pub fn visit_binary_expr_mut(v: &mut impl VisitMut, node: &mut BinaryExpr) {
    v.visit_expr_mut(&mut node.left);
    v.visit_expr_mut(&mut node.right);
}

pub fn visit_block_expr_mut(v: &mut impl VisitMut, node: &mut BlockExpr) {
    for stmt in &mut node.stmts {
        v.visit_stmt_mut(stmt);
    }
    if let Some(eval_expr) = &mut node.eval_expr {
        v.visit_expr_mut(eval_expr);
    }
}

pub fn visit_call_expr_mut(v: &mut impl VisitMut, node: &mut CallExpr) {
    v.visit_expr_mut(&mut node.caller);
    for arg in &mut node.args {
        v.visit_expr_mut(arg);
    }
}

pub fn visit_expr_mut(v: &mut impl VisitMut, node: &mut Expr) {
    match &mut node.kind {
        ExprKind::Unit => v.visit_unit_expr_mut(),
        ExprKind::Integer(value) => v.visit_int_expr_mut(value),
        ExprKind::Bool(value) => v.visit_bool_expr_mut(value),
        ExprKind::VarIdent(node) => v.visit_var_ident_expr_mut(node),
        ExprKind::Unary(node) => v.visit_unary_expr_mut(node),
        ExprKind::Binary(node) => v.visit_binary_expr_mut(node),
        ExprKind::Call(node) => v.visit_call_expr_mut(node),
        ExprKind::Block(node) => v.visit_block_expr_mut(node),
        ExprKind::Return(node) => v.visit_return_expr_mut(node),
        ExprKind::If(node) => v.visit_if_expr_mut(node),
        ExprKind::Loop(node) => v.visit_loop_expr_mut(node),
        _ => todo!(),
    };
}

pub fn visit_expr_stmt_mut(v: &mut impl VisitMut, node: &mut ExprStmt) {
    v.visit_expr_mut(&mut node.expr);
}

pub fn visit_func_mut(v: &mut impl VisitMut, node: &mut Function) {
    v.visit_block_expr_mut(&mut node.body);
}

pub fn visit_if_expr_mut(v: &mut impl VisitMut, node: &mut IfExpr) {
    v.visit_expr_mut(&mut node.cond);
    v.visit_block_expr_mut(&mut node.then);
    if let Some(else_) = &mut node.else_ {
        v.visit_block_expr_mut(else_);
    }
}

pub fn visit_item_mut(v: &mut impl VisitMut, node: &mut Item) {
    match node {
        Item::Function(node) => v.visit_func_mut(node),
        Item::ExternFunction(node) => v.visit_extern_func_mut(node),
        _ => todo!(),
    }
}

pub fn visit_let_stmt_mut(v: &mut impl VisitMut, node: &mut LetStmt) {
    v.visit_expr_mut(&mut node.value);
}

pub fn visit_loop_expr_mut(v: &mut impl VisitMut, node: &mut LoopExpr) {
    v.visit_block_expr_mut(&mut node.body);
}

pub fn visit_return_expr_mut(v: &mut impl VisitMut, node: &mut ReturnExpr) {
    if let Some(node) = &mut node.expr {
        v.visit_expr_mut(node);
    }
}

pub fn visit_stmt_mut(v: &mut impl VisitMut, node: &mut Stmt) {
    match node {
        Stmt::Expr(node) => v.visit_expr_stmt_mut(node),
        Stmt::Let(node) => v.visit_let_stmt_mut(node),
    }
}

pub fn visit_unary_expr_mut(v: &mut impl VisitMut, node: &mut UnaryExpr) {
    v.visit_expr_mut(&mut node.expr);
}
