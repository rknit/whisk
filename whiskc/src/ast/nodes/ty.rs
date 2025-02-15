use crate::ast::{
    location::{Locatable, Located, Span},
    parsing::token::{Delimiter, Keyword, Operator},
};

use super::{attributes::Attributes, punctuate::Punctuated};

#[derive(Debug, Clone)]
pub struct TypeDecl {
    pub attributes: Attributes,
    pub ty_tok: Located<Keyword>,
    pub name: Located<String>,
    pub assign_tok: Located<Operator>,
    pub kind: TypeDeclKind,
    pub semi_tok: Located<Delimiter>,
}

#[derive(Debug, Clone)]
pub enum TypeDeclKind {
    Type(Type),
    Struct(Struct),
}
impl Locatable for TypeDeclKind {
    fn get_location(&self) -> Span {
        match self {
            TypeDeclKind::Type(v) => v.get_location(),
            TypeDeclKind::Struct(v) => v.get_location(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Primitive(Located<PrimType>),
    Ident(Located<String>),
}
impl Locatable for Type {
    fn get_location(&self) -> Span {
        match self {
            Type::Primitive(ty) => ty.1,
            Type::Ident(ty) => ty.1,
        }
    }
}
impl From<Type> for TypeDeclKind {
    fn from(value: Type) -> Self {
        Self::Type(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimType {
    Unit,
    Int,
    Bool,
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub struct_tok: Located<Keyword>,
    pub brace_open_tok: Located<Delimiter>,
    pub fields: Punctuated<Field>,
    pub brace_close_tok: Located<Delimiter>,
}
impl Locatable for Struct {
    fn get_location(&self) -> Span {
        Span::combine(self.struct_tok.1, self.brace_close_tok.1)
    }
}
impl From<Struct> for TypeDeclKind {
    fn from(value: Struct) -> Self {
        Self::Struct(value)
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: Located<String>,
    pub ty: Type,
}
impl Locatable for Field {
    fn get_location(&self) -> Span {
        Span::combine(self.name.1, self.ty.get_location())
    }
}
