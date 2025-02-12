mod common;
mod symbol_id;
mod symbol_table;
pub mod ty;

pub use symbol_id::*;
pub use symbol_table::SymbolTable;

pub use self::symbol_table::{
    Block as BlockSymbol, Function as FuncSymbol, Type as TypeSymbol, Variable as VarSymbol,
};
