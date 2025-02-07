use super::{BlockSymbol, FuncSymbol, SymbolTable, TypeSymbol, VarSymbol};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeId(pub(super) u64);
impl<'a> TypeId {
    pub fn sym(&self, table: &'a mut SymbolTable) -> TypeSymbol<'a> {
        TypeSymbol::new(table, *self)
    }
}
impl From<u64> for TypeId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FuncId(pub(super) u64);
impl<'a> FuncId {
    pub fn sym(&self, table: &'a mut SymbolTable) -> FuncSymbol<'a> {
        FuncSymbol::new(table, *self)
    }
}
impl From<u64> for FuncId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub(super) u64);
impl<'a> BlockId {
    pub fn sym(&self, table: &'a mut SymbolTable) -> BlockSymbol<'a> {
        BlockSymbol::new(table, *self)
    }
}
impl From<u64> for BlockId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VarId(pub(super) BlockId, pub(super) u64);
impl<'a> VarId {
    pub fn sym(&self, table: &'a mut SymbolTable) -> VarSymbol<'a> {
        VarSymbol::new(table, *self)
    }
}
