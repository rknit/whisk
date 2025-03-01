use core::fmt;
use std::collections::HashMap;

use crate::interner::StringInterner;

use super::{
    common::{inject_symbol_table, Common, CommonType},
    BlockId, BlockSymbol, FuncId, FuncSymbol, TypeId, TypeSymbol, VarId, VarSymbol,
};

#[derive(Default, Clone)]
pub struct SymbolTable {
    pub(super) types: HashMap<TypeId, TypeSymbol>,
    pub(super) funcs: HashMap<FuncId, FuncSymbol>,
    pub(super) blocks: HashMap<BlockId, BlockSymbol>,
    pub(super) vars: HashMap<VarId, VarSymbol>,
    pub(super) interner: StringInterner,
    pub(super) blk_counter: u64,
    common: Option<Common>,
}
impl fmt::Debug for SymbolTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SymbolTable")
            .field("types", &self.types)
            .field("funcs", &self.funcs)
            .field("blocks", &self.blocks)
            .field("vars", &self.vars)
            // .field("interner", &self.interner)
            .field("blk_counter", &self.blk_counter)
            .finish()
    }
}
impl SymbolTable {
    pub fn new() -> Self {
        let mut table = Self::default();
        table.common = Some(inject_symbol_table(&mut table));
        table
    }

    /// Add the type to the type symbol table, returning its id if there is no name collision.
    /// None is returned if there is a type with the same name presented in the table.
    pub fn new_type(&mut self, name: String) -> Option<TypeId> {
        let tyid: TypeId = self.interner.intern(&name).into();
        if self.types.contains_key(&tyid) {
            return None;
        }
        self.types.insert(
            tyid,
            TypeSymbol {
                id: tyid,
                name,
                kind: None,
            },
        );
        Some(tyid)
    }

    pub fn get_type_by_name(&self, name: &str) -> Option<&TypeSymbol> {
        self.get_type_id(name).map(|v| v.sym(self))
    }

    pub fn get_type_by_name_mut(&mut self, name: &str) -> Option<&mut TypeSymbol> {
        self.get_type_id(name).map(|v| v.sym_mut(self))
    }

    pub fn get_type_id(&self, name: &str) -> Option<TypeId> {
        let tyid: TypeId = self.interner.get(name)?.into();
        if self.types.contains_key(&tyid) {
            Some(tyid)
        } else {
            None
        }
    }

    /// Add the function to the function symbol table, returning its id if there is no name collision.
    /// None is returned if there is a function with the same name presented in the table.
    pub fn new_function(&mut self, name: String) -> Option<FuncId> {
        let fid: FuncId = self.interner.intern(&name).into();
        if self.funcs.contains_key(&fid) {
            return None;
        }
        self.funcs.insert(
            fid,
            FuncSymbol {
                id: fid,
                name,
                params: vec![],
                ret_ty: Default::default(),
                entry_block: Default::default(),
            },
        );
        Some(fid)
    }

    pub fn get_function_by_name(&self, name: &str) -> Option<&FuncSymbol> {
        self.get_function_id(name).map(|v| v.sym(self))
    }

    pub fn get_function_by_name_mut(&mut self, name: &str) -> Option<&mut FuncSymbol> {
        self.get_function_id(name).map(|v| v.sym_mut(self))
    }

    pub fn get_function_id(&self, name: &str) -> Option<FuncId> {
        let fid: FuncId = self.interner.get(name)?.into();
        if self.funcs.contains_key(&fid) {
            Some(fid)
        } else {
            None
        }
    }

    pub fn new_block(&mut self, parent_func: FuncId) -> BlockId {
        let bid: BlockId = self.blk_counter.into();
        self.blk_counter += 1;
        self.blocks.insert(
            bid,
            BlockSymbol {
                id: bid,
                func: parent_func,
                parent_block: None,
            },
        );
        bid
    }

    fn get_block(&self, block: BlockId) -> Option<&BlockSymbol> {
        self.blocks.get(&block)
    }

    /// Add the variable to the variable table, returning its id if there is no name collision in
    /// its parent block.
    /// None is returned if there is a variable with the same name presented in the same block, or
    /// the parent block id is an invalid id.
    pub fn new_variable(&mut self, name: String, parent_block: BlockId) -> Option<VarId> {
        let vid = VarId(parent_block, self.interner.intern(&name));
        if !self.blocks.contains_key(&parent_block) || self.vars.contains_key(&vid) {
            return None;
        }
        self.vars.insert(
            vid,
            VarSymbol {
                id: vid,
                block: parent_block,
                name,
                ty: Default::default(),
            },
        );
        Some(vid)
    }

    pub fn get_variable_by_name_mut(
        &mut self,
        starting_block: BlockId,
        name: &str,
    ) -> Option<&mut VarSymbol> {
        self.get_variable_id_by_name(starting_block, name)
            .map(|v| v.sym_mut(self))
    }

    pub fn get_variable_id_by_name(
        &self,
        mut starting_block: BlockId,
        name: &str,
    ) -> Option<VarId> {
        let id: u64 = self.interner.get(name)?;
        let mut vid;
        while let Some(block) = self.get_block(starting_block) {
            vid = VarId(block.id, id);
            if self.vars.contains_key(&vid) {
                return Some(vid);
            }
            if let Some(parent) = block.parent_block {
                starting_block = parent;
            } else {
                break;
            }
        }
        None
    }

    pub fn common(&self) -> &Common {
        // common should have been initialized when creating the table,
        // so we can use unwrap_unchecked().
        unsafe { self.common.as_ref().unwrap_unchecked() }
    }

    pub fn common_type(&self) -> &CommonType {
        &self.common().ty
    }

    pub fn is_type_coercible(&self, from: TypeId, to: TypeId) -> bool {
        if from == self.common_type().never {
            true
        } else {
            from == to
        }
    }

    pub fn is_type_symmetric(&self, left: TypeId, right: TypeId) -> bool {
        self.is_type_coercible(left, right) && self.is_type_coercible(right, left)
    }

    pub fn compare_type_asymmetric(&self, left: TypeId, right: TypeId) -> Option<TypeId> {
        if self.is_type_symmetric(left, right) {
            Some(left)
        } else if self.is_type_coercible(left, right) {
            Some(right)
        } else if self.is_type_coercible(right, left) {
            Some(left)
        } else {
            None
        }
    }
}
