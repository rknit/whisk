use core::fmt;
use std::mem::discriminant;

use crate::symbol_table::{Symbol, SymbolID, SymbolTable};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Primitive(PrimType),
    Function(FuncType),
}
impl Type {
    pub fn is_numeric_ty(&self) -> bool {
        matches!(self, Self::Primitive(PrimType::Integer))
    }

    pub fn is_ord_ty(&self) -> bool {
        matches!(self, Type::Primitive(PrimType::Integer))
    }

    pub fn to_string(&self, symbol_table: &SymbolTable) -> String {
        match self {
            Type::Primitive(ty) => format!("{}", ty),
            Type::Function(ty) => ty.to_string(symbol_table),
        }
    }
}
impl From<PrimType> for Type {
    fn from(value: PrimType) -> Self {
        Self::Primitive(value)
    }
}
impl From<FuncType> for Type {
    fn from(value: FuncType) -> Self {
        Self::Function(value)
    }
}
impl fmt::Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Primitive(ty) => write!(f, "{}", ty),
            Type::Function(ty) => write!(f, "{:?}", ty),
        }
    }
}

#[derive(Clone, Copy, Eq)]
pub enum PrimType {
    Never,
    Unit,
    Bool,
    Integer, // i64
}
impl PartialEq for PrimType {
    fn eq(&self, other: &Self) -> bool {
        let self_d = discriminant(self);
        let other_d = discriminant(other);
        let never_d = discriminant(&Self::Never);
        if self_d == never_d || other_d == never_d {
            true
        } else {
            self_d == other_d
        }
    }
}
impl fmt::Display for PrimType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PrimType::Never => "never",
                PrimType::Unit => "()",
                PrimType::Bool => "bool",
                PrimType::Integer => "int",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FuncType(pub SymbolID);
impl FuncType {
    pub fn to_string(&self, symbol_table: &SymbolTable) -> String {
        let Symbol::Function(func_sym) = symbol_table.get_symbol(self.0).unwrap() else {
            panic!("symbol is not a function");
        };
        let params_str = func_sym
            .get_params()
            .iter()
            .map(|v| v.1.to_string(symbol_table))
            .collect::<Vec<String>>()
            .join(", ");
        format!(
            "func({}){} <{}>",
            params_str,
            func_sym.get_return_type().to_string(symbol_table),
            func_sym.get_name()
        )
    }
}
