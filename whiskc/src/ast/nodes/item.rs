use core::fmt;

use super::func::{ExternFunction, Function};

#[derive(Clone)]
pub enum Item {
    Function(Function),
    ExternFunction(ExternFunction),
}
impl fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Function(function) => write!(f, "{:#?}", function),
            Item::ExternFunction(extern_function) => write!(f, "{:#?}", extern_function),
        }
    }
}
