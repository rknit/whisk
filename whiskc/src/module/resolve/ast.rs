use crate::{
    ast,
    module::{nodes::item::Item, resolve::Record},
};

use super::Resolve;

impl Resolve<(), Vec<Item>> for ast::AST {
    fn resolve(&self, ctx: &mut super::ResolveContext, _: ()) -> Vec<Item> {
        let mut ok_indexes = Vec::new();

        // record types first since other items need types in their components.
        for (i, item) in self.items.iter().enumerate() {
            match item {
                ast::nodes::item::Item::TypeDecl(_) => {
                    if item.record(ctx, ()) {
                        ok_indexes.push(i);
                    }
                }
                _ => continue,
            }
        }

        for (i, item) in self.items.iter().enumerate() {
            match item {
                ast::nodes::item::Item::Function(_) | ast::nodes::item::Item::ExternFunction(_) => {
                    if item.record(ctx, ()) {
                        ok_indexes.push(i);
                    }
                }
                _ => continue,
            }
        }

        let mut items = Vec::new();

        // no need to separately resolve items as they had been ordered in the record phase.
        for index in ok_indexes {
            let Some(item) = self.items[index].resolve(ctx, ()) else {
                continue;
            };
            items.push(item);
        }
        items
    }
}
