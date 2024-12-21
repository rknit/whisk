use super::func::{ExternFunction, Function};

#[derive(Debug, Clone)]
pub enum Item {
    Function(Box<Function>),
    ExternFunction(Box<ExternFunction>),
}
