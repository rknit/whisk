use wsk_vm::Inst;

use super::Codegen;

use crate::{ast_resolved::nodes as ast, ty::PrimType};

impl Codegen for ast::func::Function {
    fn codegen(&self, ctx: &mut super::Context) -> Result<(), super::CodegenError> {
        ctx.set_current_fi(self.sig.sym_id);

        for param in &self.sig.params {
            ctx.push_local(Some(param.sym_id));
        }

        self.body.codegen(ctx)?;

        if self.sig.ret_ty == PrimType::Unit.into() {
            let func = ctx.get_current_fi_mut();
            if !matches!(func.get_insts()[..], [.., Inst::Ret]) {
                func.push_inst(Inst::Ret);
            }
        }

        ctx.unset_current_fi();
        Ok(())
    }
}
