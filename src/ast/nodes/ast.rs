use super::item::Item;

#[derive(Debug, Clone)]
pub struct AST {
    pub items: Vec<Item>,
}
