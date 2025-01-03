use crate::symbol_table::SymbolTable;

use super::item::Item;

#[derive(Debug, Clone)]
pub struct ResolvedAST {
    pub sym_table: SymbolTable,
    pub items: Vec<Item>,
}
