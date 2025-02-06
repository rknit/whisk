use crate::{old_symbol_table::SymbolTable, symbol};

use super::item::Item;

#[derive(Debug, Clone)]
pub struct Module {
    pub sym_table_old: SymbolTable,
    pub sym_table: symbol::SymbolTable,
    pub items: Vec<Item>,
}
