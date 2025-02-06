#![allow(dead_code)]

use std::collections::HashMap;

use crate::interner::StringInterner;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FuncId(u64);
impl From<u64> for FuncId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(u64);
impl From<u64> for BlockId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VarId(BlockId, u64);

#[derive(Debug, Default, Clone)]
pub struct SymbolTable {
    funcs: HashMap<FuncId, Function>,
    blocks: HashMap<BlockId, Block>,
    vars: HashMap<VarId, Variable>,
    interner: StringInterner,
    blk_counter: u64,
}
impl SymbolTable {
    /// Add the function to the function symbol table, returning its id if there is no name collision.
    /// None is returned if there is a function with the same name presented in the table.
    pub fn new_function(&mut self, name: String, arity: usize) -> Option<FuncId> {
        let fid: FuncId = self.interner.intern(&name).into();
        if self.funcs.contains_key(&fid) {
            return None;
        }
        self.funcs.insert(fid, Function::new(name, arity));
        Some(fid)
    }

    pub fn new_block(&mut self, parent_func: FuncId) -> BlockId {
        let bid: BlockId = self.blk_counter.into();
        self.blk_counter += 1;
        self.blocks.insert(bid, Block::new(bid, parent_func));
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
        self.vars.insert(vid, Variable::new(name, parent_block));
        Some(vid)
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    params: Vec<Param>,
}
impl Function {
    fn new(name: String, arity: usize) -> Self {
        Self {
            name,
            params: vec![Param::default(); arity],
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_param_name(&mut self, index: usize, name: String) -> Option<&mut Self> {
        self.params.get_mut(index)?.name = name;
        Some(self)
    }

    pub fn get_param(&self, index: usize) -> Option<&Param> {
        self.params.get(index)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Param {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Block {
    id: BlockId, // added an index field to identify blocks
    func: FuncId,
    parent_block: Option<BlockId>,
}
impl Block {
    fn new(id: BlockId, parent_func: FuncId) -> Self {
        Self {
            id,
            func: parent_func,
            parent_block: None,
        }
    }

    pub fn set_parent_block(&mut self, block: BlockId) -> &mut Self {
        assert!(
            self.id == block,
            "cannot assign the block itself as its parent block"
        );
        self.parent_block = Some(block);
        self
    }

    pub fn get_function(&self) -> FuncId {
        self.func
    }

    pub fn get_id(&self) -> BlockId {
        self.id
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    block: BlockId,
    name: String,
}
impl Variable {
    fn new(name: String, parent_block: BlockId) -> Self {
        Self {
            block: parent_block,
            name,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_block(&self) -> BlockId {
        self.block
    }
}
