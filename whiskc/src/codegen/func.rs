use super::Codegen;

use crate::ast_resolved::nodes as ast;

impl Codegen for ast::func::Function {
    fn codegen(&self, ctx: &mut super::Context) -> Result<(), super::CodegenError> {
        ctx.set_current_fi(self.sig.sym_id);

        for param in &self.sig.params {
            ctx.push_local(Some(param.sym_id));
        }

        self.body.codegen(ctx)?;

        ctx.unset_current_fi();
        Ok(())
    }
}
