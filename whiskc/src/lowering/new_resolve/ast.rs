use crate::{
    ast,
    lowering::{new_resolve::Record, nodes::item::Item},
};

use super::Resolve;

impl Resolve<(), Vec<Item>> for ast::AST {
    fn resolve(&self, ctx: &mut super::ResolveContext, _: ()) -> Vec<Item> {
        for item in &self.items {
            item.record(ctx, ());
        }

        // for item in &self.items {
        //     let Some(item) = item.resolve(ctx, ()) else {
        //         continue;
        //     };
        //     items.push(item);
        // }

        Vec::new()
    }
}
