pub mod ast;
pub mod codegen;
pub mod compile;
mod interner;
pub mod module;
pub mod symbol;

pub use compile::compile;
