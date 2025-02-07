use crate::{
    ast,
    lowering::{new_resolve::Record, nodes::item::Item},
};

use super::Resolve;

impl Resolve<(), Option<Vec<Item>>> for ast::AST {
    fn resolve(&self, ctx: &mut super::ResolveContext, _: ()) -> Option<Vec<Item>> {
        for item in &self.items {
            item.record(ctx, ());
        }

        let mut items = Vec::new();
        for item in &self.items {
            let Some(item) = item.resolve(ctx, ()) else {
                continue;
            };
            items.push(item);
        }

        Some(items)
    }
}
