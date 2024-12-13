use super::item::Item;

#[derive(Debug, Clone)]
pub struct ResolvedAST {
    pub items: Vec<Item>,
}
