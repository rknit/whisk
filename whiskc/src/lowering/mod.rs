pub mod errors;
mod new_resolve;
pub mod nodes;
mod resolve;
pub mod visit;

pub use new_resolve::resolve;
pub use nodes::module::Module;
pub use resolve::module::resolve as old_resolve;
