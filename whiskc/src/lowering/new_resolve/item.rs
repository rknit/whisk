use super::{Record, Resolve, ResolveContext};

use crate::{ast::nodes as ast, lowering::nodes::item::Item};

impl Record for ast::item::Item {
    fn record(&self, ctx: &mut ResolveContext, _: ()) {
        match self {
            ast::item::Item::Function(v) => v.sig.record(ctx, ()),
            ast::item::Item::ExternFunction(v) => v.sig.record(ctx, ()),
            ast::item::Item::TypeDecl(v) => v.record(ctx, ()),
        }
    }
}

impl Resolve<(), Option<Item>> for ast::item::Item {
    fn resolve(&self, ctx: &mut ResolveContext, _: ()) -> Option<Item> {
        match self {
            ast::item::Item::Function(_) => todo!(),
            ast::item::Item::ExternFunction(_) => todo!(),
            ast::item::Item::TypeDecl(v) => v.resolve(ctx, ()).map(Item::TypeDecl),
        }
    }
}
