use crate::lowering::nodes::{
    expr::{ExprKind, FuncIdentExpr, StructInitExpr},
    ty::TypeDecl,
};

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

pub trait Visit: Sized {
    fn visit_module(&mut self, node: &Module) {
        visit_module(self, node);
    }

    fn visit_binary_expr(&mut self, node: &BinaryExpr) {
        visit_binary_expr(self, node);
    }

    fn visit_block_expr(&mut self, node: &BlockExpr) {
        visit_block_expr(self, node);
    }

    fn visit_bool_expr(&mut self, _value: bool) {
        /* terminal */
    }

    fn visit_call_expr(&mut self, node: &CallExpr) {
        visit_call_expr(self, node);
    }

    fn visit_expr(&mut self, node: &Expr) {
        visit_expr(self, node);
    }

    fn visit_expr_stmt(&mut self, node: &ExprStmt) {
        visit_expr_stmt(self, node);
    }

    fn visit_extern_func(&mut self, _node: &ExternFunction) {
        /* terminal */
    }

    fn visit_func(&mut self, node: &Function) {
        visit_func(self, node);
    }

    fn visit_var_ident_expr(&mut self, _node: &VarIdentExpr) {
        /* terminal */
    }

    fn visit_func_ident_expr(&mut self, _node: &FuncIdentExpr) {
        /* terminal */
    }

    fn visit_if_expr(&mut self, node: &IfExpr) {
        visit_if_expr(self, node);
    }

    fn visit_int_expr(&mut self, _value: i64) {
        /* terminal */
    }

    fn visit_item(&mut self, node: &Item) {
        visit_item(self, node);
    }

    fn visit_let_stmt(&mut self, node: &LetStmt) {
        visit_let_stmt(self, node);
    }

    fn visit_loop_expr(&mut self, node: &LoopExpr) {
        visit_loop_expr(self, node);
    }

    fn visit_return_expr(&mut self, node: &ReturnExpr) {
        visit_return_expr(self, node);
    }

    fn visit_stmt(&mut self, node: &Stmt) {
        visit_stmt(self, node);
    }

    fn visit_struct_init_expr(&mut self, _node: &StructInitExpr) {
        /* terminal */
    }

    fn visit_type_decl(&mut self, _node: &TypeDecl) {
        /* terminal */
    }

    fn visit_unary_expr(&mut self, node: &UnaryExpr) {
        visit_unary_expr(self, node);
    }

    fn visit_unit_expr(&mut self) {
        /* terminal */
    }
}

pub fn visit_module(v: &mut impl Visit, node: &Module) {
    for item in &node.items {
        v.visit_item(item);
    }
}

pub fn visit_binary_expr(v: &mut impl Visit, node: &BinaryExpr) {
    v.visit_expr(&node.left);
    v.visit_expr(&node.right);
}

pub fn visit_block_expr(v: &mut impl Visit, node: &BlockExpr) {
    for stmt in &node.stmts {
        v.visit_stmt(stmt);
    }
    if let Some(eval_expr) = &node.eval_expr {
        v.visit_expr(eval_expr);
    }
}

pub fn visit_call_expr(v: &mut impl Visit, node: &CallExpr) {
    v.visit_expr(&node.caller);
    for arg in &node.args {
        v.visit_expr(arg);
    }
}

pub fn visit_expr(v: &mut impl Visit, node: &Expr) {
    match &node.kind {
        ExprKind::Unit => v.visit_unit_expr(),
        ExprKind::Integer(value) => v.visit_int_expr(*value),
        ExprKind::Bool(value) => v.visit_bool_expr(*value),
        ExprKind::VarIdent(node) => v.visit_var_ident_expr(node),
        ExprKind::FuncIdent(node) => v.visit_func_ident_expr(node),
        ExprKind::Unary(node) => v.visit_unary_expr(node),
        ExprKind::Binary(node) => v.visit_binary_expr(node),
        ExprKind::Call(node) => v.visit_call_expr(node),
        ExprKind::Block(node) => v.visit_block_expr(node),
        ExprKind::Return(node) => v.visit_return_expr(node),
        ExprKind::If(node) => v.visit_if_expr(node),
        ExprKind::Loop(node) => v.visit_loop_expr(node),
        ExprKind::StructInit(node) => v.visit_struct_init_expr(node),
    };
}

pub fn visit_expr_stmt(v: &mut impl Visit, node: &ExprStmt) {
    v.visit_expr(&node.expr);
}

pub fn visit_func(v: &mut impl Visit, node: &Function) {
    v.visit_block_expr(&node.body);
}

pub fn visit_if_expr(v: &mut impl Visit, node: &IfExpr) {
    v.visit_expr(&node.cond);
    v.visit_block_expr(&node.then);
    if let Some(else_) = &node.else_ {
        v.visit_block_expr(else_);
    }
}

pub fn visit_item(v: &mut impl Visit, node: &Item) {
    match node {
        Item::Function(node) => v.visit_func(node),
        Item::ExternFunction(node) => v.visit_extern_func(node),
        Item::TypeDecl(node) => v.visit_type_decl(node),
    }
}

pub fn visit_let_stmt(v: &mut impl Visit, node: &LetStmt) {
    v.visit_expr(&node.value);
}

pub fn visit_loop_expr(v: &mut impl Visit, node: &LoopExpr) {
    v.visit_block_expr(&node.body);
}

pub fn visit_return_expr(v: &mut impl Visit, node: &ReturnExpr) {
    if let Some(node) = &node.expr {
        v.visit_expr(node);
    }
}

pub fn visit_stmt(v: &mut impl Visit, node: &Stmt) {
    match node {
        Stmt::Expr(node) => v.visit_expr_stmt(node),
        Stmt::Let(node) => v.visit_let_stmt(node),
    }
}

pub fn visit_unary_expr(v: &mut impl Visit, node: &UnaryExpr) {
    v.visit_expr(&node.expr);
}
