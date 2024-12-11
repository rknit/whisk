use super::func::{ExternFunction, Function};

#[derive(Debug, Clone)]
pub enum Item {
    Function(Function),
    ExternFunction(ExternFunction),
}
impl From<Function> for Item {
    fn from(value: Function) -> Self {
        Self::Function(value)
    }
}
impl From<ExternFunction> for Item {
    fn from(value: ExternFunction) -> Self {
        Self::ExternFunction(value)
    }
}
