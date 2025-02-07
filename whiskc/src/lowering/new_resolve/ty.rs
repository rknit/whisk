use crate::ast::nodes as ast;

use super::Record;

impl Record for ast::ty::TypeDecl {
    fn record(&self, ctx: &mut super::ResolveContext, _: ()) {
        if ctx.table.new_type(self.name.0.clone()).is_none() {
            todo!("report error");
        }
    }
}
