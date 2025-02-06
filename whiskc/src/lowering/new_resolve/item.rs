use super::{Resolve, Track};

use crate::ast::nodes as ast;

impl Track for ast::item::Item {
    fn track(&self, ctx: &mut super::ResolveContext, _: ()) {
        todo!()
    }
}

impl Resolve for ast::item::Item {
    fn resolve(&self, ctx: &mut super::ResolveContext, _: ()) {
        todo!()
    }
}
