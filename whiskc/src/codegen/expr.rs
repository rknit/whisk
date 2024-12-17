use wsk_vm::Inst;

use crate::{
    ast_resolved::nodes::expr::{Expr, ExprKind},
    ty::Type,
};

use super::{Codegen, CodegenError, Context};

impl Codegen for Expr {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError> {
        match self.get_kind() {
            ExprKind::Integer(v) => v.codegen(ctx, self.get_type()),
            ExprKind::Bool(v) => v.codegen(ctx, self.get_type()),
            ExprKind::Identifier(_) => todo!(),
            ExprKind::Unary(unary_expr) => todo!(),
            ExprKind::Binary(binary_expr) => todo!(),
            ExprKind::Call(call_expr) => todo!(),
        }
    }
}

impl ExprCodegen for i64 {
    fn codegen(&self, ctx: &mut Context, ty: Type) -> Result<(), CodegenError> {
        ctx.get_current_fi_mut()
            .push_inst(Inst::Push((*self).into()));
        ctx.push_local(None);
        Ok(())
    }
}

impl ExprCodegen for bool {
    fn codegen(&self, ctx: &mut Context, ty: Type) -> Result<(), CodegenError> {
        ctx.get_current_fi_mut()
            .push_inst(Inst::Push((*self).into()));
        ctx.push_local(None);
        Ok(())
    }
}

trait ExprCodegen {
    fn codegen(&self, ctx: &mut Context, ty: Type) -> Result<(), CodegenError>;
}
