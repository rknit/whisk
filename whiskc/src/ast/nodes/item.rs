use super::{
    func::{ExternFunction, Function},
    ty::TypeDecl,
};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum Item {
    Function(Function),
    ExternFunction(ExternFunction),
    TypeDecl(TypeDecl),
}
