use wsk_vm::Inst;

use super::{expr::ExprCodegen, Codegen};

use crate::module::nodes::{self as ast};

impl Codegen for ast::func::Function {
    fn codegen(&self, ctx: &mut super::Context) -> Result<(), super::CodegenError> {
        ctx.set_current_fi(self.func_id);
        let func_sym = self.func_id.sym(ctx.sym_table);

        for param_id in &func_sym.params {
            let id = ctx.get_local(*param_id);
            ctx.get_current_fi_mut().push_inst(Inst::Store(id));
        }

        self.body.codegen(ctx)?;

        if func_sym.ret_ty == ctx.sym_table.common_type().unit
            || self
                .body
                .eval_expr
                .as_ref()
                .map(|v| v.ty == ctx.sym_table.common_type().never)
                .unwrap_or(false)
        {
            let func = ctx.get_current_fi_mut();
            if !matches!(func.get_insts()[..], [.., Inst::Ret]) {
                func.push_inst(Inst::Ret);
            }
        }

        ctx.unset_current_fi();
        Ok(())
    }
}
