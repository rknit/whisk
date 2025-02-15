use crate::{
    ast,
    lowering::{nodes::item::Item, resolve::Record},
};

use super::Resolve;

impl Resolve<(), Vec<Item>> for ast::AST {
    fn resolve(&self, ctx: &mut super::ResolveContext, _: ()) -> Vec<Item> {
        let mut ok_indexes = Vec::new();
        for (i, item) in self.items.iter().enumerate() {
            if item.record(ctx, ()) {
                ok_indexes.push(i);
            }
        }

        let mut items = Vec::new();
        for index in ok_indexes {
            let Some(item) = self.items[index].resolve(ctx, ()) else {
                continue;
            };
            items.push(item);
        }
        items
    }
}
