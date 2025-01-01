use crate::{
    ast::{
        location::Located,
        parsing::token::{Delimiter, Keyword, Operator},
    },
    ty::PrimType,
};

use super::punctuate::Punctuated;

#[derive(Debug, Clone)]
pub struct TypeDecl {
    pub ty_tok: Located<Keyword>,
    pub name: Located<String>,
    pub assign_tok: Located<Operator>,
    pub kind: Type,
    pub semi_tok: Located<Delimiter>,
}

#[derive(Debug, Clone)]
pub enum Type {
    Unit(UnitType),
    Primitive(Located<PrimType>),
    Struct(Struct),
    Ident(Located<String>),
}

#[derive(Debug, Clone)]
pub struct UnitType {
    pub paren_open_tok: Located<Delimiter>,
    pub paren_close_tok: Located<Delimiter>,
}
impl From<UnitType> for Type {
    fn from(value: UnitType) -> Self {
        Self::Unit(value)
    }
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub struct_tok: Located<Keyword>,
    pub brace_open_tok: Located<Delimiter>,
    pub fields: Punctuated<Field>,
    pub brace_close_tok: Located<Delimiter>,
}
impl From<Struct> for Type {
    fn from(value: Struct) -> Self {
        Self::Struct(value)
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: Located<String>,
    pub ty: Type,
}
