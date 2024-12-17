use crate::ast_resolved::nodes::stmt::{Block, Stmt};

use super::Codegen;

impl Codegen for Stmt {
    fn codegen(&self, ctx: &mut super::Context) -> Result<(), super::CodegenError> {
        match self {
            Stmt::Block(stmt) => stmt.codegen(ctx),
            Stmt::Expr(stmt) => todo!(),
            Stmt::Assign(stmt) => todo!(),
            Stmt::Let(stmt) => todo!(),
            Stmt::If(stmt) => todo!(),
            Stmt::Return(stmt) => todo!(),
        }
    }
}

impl Codegen for Block {
    fn codegen(&self, ctx: &mut super::Context) -> Result<(), super::CodegenError> {
        ctx.push_bound();

        for stmt in &self.stmts {
            stmt.codegen(ctx)?;
        }

        ctx.pop_bound();
        Ok(())
    }
}
