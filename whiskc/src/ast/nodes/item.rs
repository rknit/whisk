use super::func::{ExternFunction, Function};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum Item {
    Function(Function),
    ExternFunction(ExternFunction),
}
