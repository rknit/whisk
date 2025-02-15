use std::mem::size_of;

use super::{SymbolTable, TypeId};

#[derive(Debug, Clone)]
pub enum TypeKind {
    Primitive(Primitive),
    Struct(StructType),
    Ident(TypeId),
    Alias(TypeId),
}
impl TypeKind {
    pub fn get_size(&self, table: &SymbolTable) -> Option<usize> {
        match self {
            TypeKind::Primitive(v) => Some(v.get_size()),
            TypeKind::Struct(v) => v.get_size(table),
            TypeKind::Ident(v) => v.sym(table).get_size(table),
            TypeKind::Alias(v) => v.sym(table).get_size(table),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Primitive {
    Never,
    Unit,
    Bool,
    Int,
}
impl Primitive {
    pub fn get_size(&self) -> usize {
        match self {
            Self::Never | Self::Unit => 0,
            Self::Bool => size_of::<bool>(),
            Self::Int => size_of::<i64>(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StructType {
    pub fields: Vec<(String, TypeId)>,
}
impl StructType {
    pub fn get_size(&self, table: &SymbolTable) -> Option<usize> {
        let mut sz = 0;
        for field in &self.fields {
            sz += field.1.sym(table).get_size(table)?;
        }
        Some(sz)
    }
}
