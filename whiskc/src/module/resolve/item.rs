use super::{Record, Resolve, ResolveContext};

use crate::{ast::nodes as ast, module::nodes::item::Item};

impl Record<(), bool> for ast::item::Item {
    fn record(&self, ctx: &mut ResolveContext, _: ()) -> bool {
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
            ast::item::Item::Function(v) => v.resolve(ctx, ()).map(Item::Function),
            ast::item::Item::ExternFunction(v) => v.resolve(ctx, ()).map(Item::ExternFunction),
            ast::item::Item::TypeDecl(v) => v.resolve(ctx, ()).map(Item::TypeDecl),
        }
    }
}
