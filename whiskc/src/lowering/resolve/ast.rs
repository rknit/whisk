use crate::{
    ast,
    lowering::{nodes::item::Item, resolve::Record},
};

use super::Resolve;

impl Resolve<(), Vec<Item>> for ast::AST {
    fn resolve(&self, ctx: &mut super::ResolveContext, _: ()) -> Vec<Item> {
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
        items
    }
}
