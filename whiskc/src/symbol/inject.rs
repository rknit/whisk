use strum::IntoEnumIterator;

use crate::ast::parsing::token::TypeKeyword;

use super::SymbolTable;

pub fn inject_symbol_table(table: &mut SymbolTable) {
    inject_primitive_types(table);
}

fn inject_primitive_types(table: &mut SymbolTable) {
    for kw in TypeKeyword::iter() {
        table.new_type(kw.to_string().to_lowercase()).unwrap();
    }
}
