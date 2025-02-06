use crate::{ast, lowering::nodes::item::Item};

use super::Resolve;

impl Resolve<(), Option<Vec<Item>>> for ast::AST {
    fn resolve(&self, ctx: &mut super::ResolveContext, _: ()) -> Option<Vec<Item>> {
        todo!()
    }
}
