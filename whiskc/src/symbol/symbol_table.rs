#![allow(dead_code)]

use std::collections::HashMap;

use crate::{
    interner::StringInterner,
    symbol::{BlockSymbol, FuncSymbol, VarSymbol},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemId {
    Func(FuncId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FuncId(u64);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(u64);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VarId(BlockId, u64);
impl<'a> VarId {
    pub fn sym(&self, table: &'a mut SymbolTable) -> VarSymbol<'a> {
        VarSymbol::new(table, *self)
    }
}

#[derive(Debug, Default, Clone)]
pub struct SymbolTable {
    pub(super) funcs: HashMap<FuncId, Function>,
    pub(super) blocks: HashMap<BlockId, Block>,
    pub(super) vars: HashMap<VarId, Variable>,
    pub(super) interner: StringInterner,
    pub(super) blk_counter: u64,
}
impl SymbolTable {
    /// Add the function to the function symbol table, returning its id if there is no name collision.
    /// None is returned if there is a function with the same name presented in the table.
    pub fn new_function(&mut self, name: String, arity: usize) -> Option<FuncId> {
        let fid: FuncId = self.interner.intern(&name).into();
        if self.funcs.contains_key(&fid) {
            return None;
        }
        self.funcs.insert(
            fid,
            Function {
                name,
                params: vec![Param::default(); arity],
            },
        );
        Some(fid)
    }

    pub fn get_function_by_name(&mut self, name: &str) -> Option<&Function> {
        self.funcs.get(&self.interner.intern(name).into())
    }

    pub fn new_block(&mut self, parent_func: FuncId) -> BlockId {
        let bid: BlockId = self.blk_counter.into();
        self.blk_counter += 1;
        self.blocks.insert(
            bid,
            Block {
                id: bid,
                func: parent_func,
                parent_block: None,
            },
        );
        bid
    }

    pub fn get_block(&self, block_id: BlockId) -> Option<&Block> {
        self.blocks.get(&block_id)
    }

    /// Add the variable to the variable table, returning its id if there is no name collision in
    /// its parent block.
    /// None is returned if there is a variable with the same name presented in the same block, or
    /// the parent block id is an invalid id.
    pub fn new_variable(&mut self, name: String, parent_block: BlockId) -> Option<VarId> {
        let vid = VarId(parent_block, self.interner.intern(&name));
        if self.get_block(parent_block).is_none() || self.vars.contains_key(&vid) {
            return None;
        }
        self.vars.insert(
            vid,
            Variable {
                block: parent_block,
                name,
            },
        );
        Some(vid)
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub(super) name: String,
    pub(super) params: Vec<Param>,
}

#[derive(Debug, Default, Clone)]
pub struct Param {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub(super) id: BlockId, // added an id field to identify the block
    pub(super) func: FuncId,
    pub(super) parent_block: Option<BlockId>,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub(super) block: BlockId,
    pub(super) name: String,
}
