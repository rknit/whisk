use super::Record;

use crate::ast::nodes as ast;

impl Record for ast::func::FunctionSig {
    fn record(&self, ctx: &mut super::ResolveContext, _: ()) {
        if ctx
            .table
            .new_function(self.name.0.clone(), self.params.items.len())
            .is_none()
        {
            todo!("report error");
        };
    }
}
