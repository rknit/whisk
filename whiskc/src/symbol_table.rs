#![allow(dead_code)]

use std::collections::HashMap;

use crate::interner::StringInterner;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FuncId(u64);
impl<'a> FuncId {
    pub fn sym(&self, table: &'a mut SymbolTable) -> FuncSymbol<'a> {
        FuncSymbol { table, id: *self }
    }
}
impl From<u64> for FuncId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

pub struct FuncSymbol<'a> {
    table: &'a mut SymbolTable,
    id: FuncId,
}
impl<'a> FuncSymbol<'a> {
    fn get(&self) -> &Function {
        self.table.funcs.get(&self.id).unwrap()
    }

    fn get_mut(&mut self) -> &mut Function {
        self.table.funcs.get_mut(&self.id).unwrap()
    }

    pub fn get_name(&self) -> &str {
        &self.get().name
    }

    pub fn set_param_name(&mut self, index: usize, name: String) -> Option<&mut Self> {
        self.get_mut().params.get_mut(index)?.name = name;
        Some(self)
    }

    pub fn get_param(&self, index: usize) -> Option<&Param> {
        self.get().params.get(index)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(u64);
impl<'a> BlockId {
    pub fn sym(&self, table: &'a mut SymbolTable) -> BlockSymbol<'a> {
        BlockSymbol { table, id: *self }
    }
}
impl From<u64> for BlockId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

pub struct BlockSymbol<'a> {
    table: &'a mut SymbolTable,
    id: BlockId,
}
impl BlockSymbol<'_> {
    fn get(&self) -> &Block {
        self.table.blocks.get(&self.id).unwrap()
    }

    fn get_mut(&mut self) -> &mut Block {
        self.table.blocks.get_mut(&self.id).unwrap()
    }

    pub fn set_parent_block(&mut self, block: BlockId) -> &mut Self {
        assert!(
            self.id == block,
            "cannot assign the block itself as its parent block"
        );
        self.get_mut().parent_block = Some(block);
        self
    }

    pub fn get_function(&self) -> FuncId {
        self.get().func
    }

    pub fn get_id(&self) -> BlockId {
        self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VarId(BlockId, u64);
impl<'a> VarId {
    pub fn sym(&self, table: &'a mut SymbolTable) -> VarSymbol<'a> {
        VarSymbol { table, id: *self }
    }
}

pub struct VarSymbol<'a> {
    table: &'a mut SymbolTable,
    id: VarId,
}
impl VarSymbol<'_> {
    fn get(&self) -> &Variable {
        self.table.vars.get(&self.id).unwrap()
    }

    fn get_mut(&mut self) -> &mut Variable {
        self.table.vars.get_mut(&self.id).unwrap()
    }

    pub fn get_name(&self) -> &str {
        &self.get().name
    }

    pub fn get_block(&self) -> BlockId {
        self.get().block
    }
}

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
        self.funcs.insert(
            fid,
            Function {
                name,
                params: vec![Param::default(); arity],
            },
        );
        Some(fid)
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
    name: String,
    params: Vec<Param>,
}

#[derive(Debug, Default, Clone)]
pub struct Param {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Block {
    id: BlockId, // added an id field to identify the block
    func: FuncId,
    parent_block: Option<BlockId>,
}

#[derive(Debug, Clone)]
pub struct Variable {
    block: BlockId,
    name: String,
}
