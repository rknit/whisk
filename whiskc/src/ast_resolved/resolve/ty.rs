use crate::ast_resolved::ResolveContext;
use crate::ast_resolved::{nodes::ty::Type, Resolve};

use crate::ast::nodes::ty as ast_ty;

impl Resolve<Type> for ast_ty::Type {
    fn resolve(&self, _ctx: &mut ResolveContext) -> Option<Type> {
        todo!()
    }
}
