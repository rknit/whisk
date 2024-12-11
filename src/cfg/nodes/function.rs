use std::{
    collections::{hash_map::Iter, HashMap},
    mem,
};

use uuid::Uuid;

use crate::symbol_table::SymbolID;

use super::{
    basic_block::{BasicBlock, BasicBlockID},
    inst::InstID,
};

#[derive(Debug)]
pub struct Function {
    func_sym_id: SymbolID,
    table_id: SymbolID,
    entry: Option<BasicBlockID>,
    bbs: HashMap<BasicBlockID, BasicBlock>,
}
impl Function {
    pub fn new(func_sym_id: SymbolID, table_id: SymbolID) -> Self {
        Self {
            func_sym_id,
            table_id,
            entry: None,
            bbs: HashMap::new(),
        }
    }

    pub fn get_symbol_id(&self) -> SymbolID {
        self.func_sym_id
    }

    pub fn get_table_id(&self) -> SymbolID {
        self.table_id
    }

    pub fn has_entry(&self) -> bool {
        self.entry.is_some()
    }

    pub fn add_block(&mut self, block: BasicBlock) -> BasicBlockID {
        let id = Uuid::new_v4();
        let res = self.bbs.insert(id, block);
        debug_assert!(res.is_none(), "duplicate ID");
        if self.entry.is_none() {
            self.entry = Some(id);
        }
        id
    }

    pub fn link_block(&mut self, from: BasicBlockID, to: BasicBlockID) {
        self.get_block_mut(from).unwrap().outgoings.insert(to);
        self.get_block_mut(to).unwrap().incomings.insert(from);
    }

    pub fn unlink_block_outgoings(&mut self, bb: BasicBlockID) {
        for outgoing in mem::take(&mut self.get_block_mut(bb).unwrap().outgoings) {
            self.get_block_mut(outgoing).unwrap().incomings.remove(&bb);
        }
        debug_assert!(self.get_block_mut(bb).unwrap().outgoings.is_empty());
    }

    /// Split the block's instructions into 2 parts: from index 0 to the index of *inst* and the index of *inst* + 1 to the last index.
    /// The first part will retain incoming branches and the latter will retain the outgoing branches.
    pub fn split_block_at(
        &mut self,
        bb: BasicBlockID,
        inst: InstID,
    ) -> Option<(BasicBlockID, BasicBlockID)> {
        let block = self.get_block_mut(bb)?;
        let index = block.insts.iter().position(|v| v.id == inst)?;

        let mut latter_block = BasicBlock::new(block.desc.clone().map(|v| v + "_latter"));
        latter_block.insts = block.insts.split_off(index + 1);
        latter_block.outgoings = mem::take(&mut block.outgoings);
        let latter_id = self.add_block(latter_block);

        Some((bb, latter_id))
    }

    pub fn get_block(&self, block: BasicBlockID) -> Option<&BasicBlock> {
        self.bbs.get(&block)
    }

    pub fn get_block_mut(&mut self, block: BasicBlockID) -> Option<&mut BasicBlock> {
        self.bbs.get_mut(&block)
    }

    pub fn get_blocks(&self) -> Iter<BasicBlockID, BasicBlock> {
        self.bbs.iter()
    }

    pub fn get_entry_block(&self) -> BasicBlockID {
        self.entry.expect("unset entry block")
    }

    pub fn is_valid_block(&self, block: BasicBlockID) -> bool {
        self.bbs.contains_key(&block)
    }
}
