use wsk_vm::Inst;

use super::{expr::ExprCodegen, Codegen};

use crate::{ast_resolved::nodes as ast, ty::PrimType};

impl Codegen for ast::func::Function {
    fn codegen(&self, ctx: &mut super::Context) -> Result<(), super::CodegenError> {
        ctx.set_current_fi(self.sig.sym_id);

        for param in &self.sig.params {
            let id = ctx.get_local(param.sym_id);
            ctx.get_current_fi_mut().push_inst(Inst::Store(id));
        }

        self.body.codegen(ctx)?;

        if self.sig.ret_ty == PrimType::Unit.into() || self.body.eval_expr.is_some() {
            let func = ctx.get_current_fi_mut();
            if !matches!(func.get_insts()[..], [.., Inst::Ret]) {
                func.push_inst(Inst::Ret);
            }
        }

        ctx.unset_current_fi();
        Ok(())
    }
}
