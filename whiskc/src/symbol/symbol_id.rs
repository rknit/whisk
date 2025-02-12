use super::{BlockSymbol, FuncSymbol, SymbolTable, TypeSymbol, VarSymbol};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeId(pub(super) u64);
impl<'a> TypeId {
    pub fn sym(&self, table: &'a SymbolTable) -> &'a TypeSymbol {
        table.types.get(self).unwrap()
    }

    pub fn sym_mut(&self, table: &'a mut SymbolTable) -> &'a mut TypeSymbol {
        table.types.get_mut(self).unwrap()
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
    pub fn sym(&self, table: &'a SymbolTable) -> &'a FuncSymbol {
        table.funcs.get(self).unwrap()
    }

    pub fn sym_mut(&self, table: &'a mut SymbolTable) -> &'a mut FuncSymbol {
        table.funcs.get_mut(self).unwrap()
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
    pub fn sym(&self, table: &'a SymbolTable) -> &'a BlockSymbol {
        table.blocks.get(self).unwrap()
    }

    pub fn sym_mut(&self, table: &'a mut SymbolTable) -> &'a mut BlockSymbol {
        table.blocks.get_mut(self).unwrap()
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
    pub fn sym(&self, table: &'a SymbolTable) -> &'a VarSymbol {
        table.vars.get(self).unwrap()
    }

    pub fn sym_mut(&self, table: &'a mut SymbolTable) -> &'a mut VarSymbol {
        table.vars.get_mut(self).unwrap()
    }
}
