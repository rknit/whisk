use core::mem::discriminant;

use crate::old_symbol_table::{SymbolID, SymbolTable};

#[derive(Debug, Clone)]
pub struct TypeDecl(pub SymbolID);

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Never,
    Unit,
    Int,
    Bool,
    Alias(SymbolID),
    Struct(SymbolID),
    Func(SymbolID),
}
impl Type {
    pub fn is_numeric_ty(&self) -> bool {
        matches!(self, Self::Int)
    }

    pub fn is_ord_ty(&self) -> bool {
        matches!(self, Type::Int)
    }

    pub fn get_size(&self, _sym_table: &SymbolTable) -> usize {
        todo!()
    }

    pub fn to_string(&self, _sym_table: &SymbolTable) -> String {
        todo!()
    }
}
impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Alias(l0), Self::Alias(r0)) => l0 == r0,
            (Self::Func(l0), Self::Func(r0)) => l0 == r0,
            (Self::Never, _) | (_, Self::Never) => true,
            _ => discriminant(self) == discriminant(other),
        }
    }
}
