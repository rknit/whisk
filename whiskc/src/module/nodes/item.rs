use super::{
    func::{ExternFunction, Function},
    ty::TypeDecl,
};

#[derive(Debug, Clone)]
pub enum Item {
    Function(Function),
    ExternFunction(ExternFunction),
    TypeDecl(TypeDecl),
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
