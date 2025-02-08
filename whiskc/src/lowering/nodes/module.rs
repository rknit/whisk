use crate::symbol::SymbolTable;

use super::item::Item;

#[derive(Debug, Clone)]
pub struct Module {
    pub sym_table: SymbolTable,
    pub items: Vec<Item>,
}
