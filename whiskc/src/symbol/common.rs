use core::fmt;

use super::{SymbolTable, TypeId};

#[derive(Default, Debug, Clone, Copy)]
pub struct Common {
    pub ty: CommonType,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct CommonType {
    pub never: TypeId,
    pub unit: TypeId,
    pub int: TypeId,
    pub bool: TypeId,
}

pub fn inject_symbol_table(table: &mut SymbolTable) -> Common {
    let common_ty = inject_primitive_types(table);
    Common { ty: common_ty }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// TODO: move this into an appropriate module.
pub enum PrimitiveType {
    Never,
    Unit,
    Int,
    Bool,
}
impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Never => "never",
                Self::Unit => "()",
                Self::Int => "int",
                Self::Bool => "bool",
            }
        )
    }
}

fn inject_primitive_types(table: &mut SymbolTable) -> CommonType {
    let mut f = |ty: PrimitiveType| table.new_type(ty.to_string()).unwrap();
    CommonType {
        never: f(PrimitiveType::Never),
        unit: f(PrimitiveType::Unit),
        int: f(PrimitiveType::Int),
        bool: f(PrimitiveType::Bool),
    }
}
