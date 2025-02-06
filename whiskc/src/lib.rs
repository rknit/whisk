pub mod ast;
pub mod codegen;
pub mod compile;
mod interner;
pub mod lowering;
pub mod old_symbol_table;
mod symbol_table;

pub use compile::compile;
