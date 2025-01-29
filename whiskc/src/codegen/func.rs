use wsk_vm::Inst;

use super::{expr::ExprCodegen, Codegen};

use crate::{
    lowering::nodes::{self as ast, ty::Type},
    symbol_table::Symbol,
};

impl Codegen for ast::func::Function {
    fn codegen(&self, ctx: &mut super::Context) -> Result<(), super::CodegenError> {
        ctx.set_current_fi(self.sym_id);
        let Symbol::Function(func_sym) = ctx.sym_table.get_symbol(self.sym_id).unwrap() else {
            unreachable!();
        };

        for param_id in func_sym.get_param_ids() {
            let id = ctx.get_local(*param_id);
            ctx.get_current_fi_mut().push_inst(Inst::Store(id));
        }

        self.body.codegen(ctx)?;

        if func_sym.get_return_type() == Type::Unit
            || self
                .body
                .eval_expr
                .as_ref()
                .map(|v| v.get_type() != Type::Never)
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
