use std::collections::HashSet;

use uuid::Uuid;

use super::inst::{Inst, InstID, TaggedInst};

pub type BasicBlockID = Uuid;

#[derive(Debug)]
pub struct BasicBlock {
    pub desc: Option<String>,
    pub(super) incomings: HashSet<BasicBlockID>,
    pub(super) outgoings: HashSet<BasicBlockID>,
    pub(super) insts: Vec<TaggedInst>,
}
impl BasicBlock {
    pub fn new(desc: Option<String>) -> Self {
        Self {
            desc,
            incomings: HashSet::new(),
            outgoings: HashSet::new(),
            insts: vec![],
        }
    }

    /// Insert *inst* before *before_inst*, or at the end of the instruction list if *None* is provided.
    /// Note: This method doesn't account for termiator instructions and must be handled by cfg::Builder
    /// struct.
    pub(in super::super) fn insert_inst(
        &mut self,
        inst: Inst,
        before_inst: Option<InstID>,
    ) -> Option<InstID> {
        if before_inst.is_none() {
            let id = Uuid::new_v4();
            self.insts.push(TaggedInst { id, inst });
            return Some(id);
        }

        let Some(index) = self
            .insts
            .iter_mut()
            .position(|v| v.id == before_inst.unwrap())
        else {
            return None;
        };

        let id = Uuid::new_v4();
        self.insts.insert(index, TaggedInst { id, inst });
        Some(id)
    }

    pub fn get_inst(&self, inst: InstID) -> Option<&TaggedInst> {
        self.get_insts().iter().find(|v| v.id == inst)
    }

    pub fn get_insts(&self) -> &Vec<TaggedInst> {
        &self.insts
    }

    pub fn is_terminated(&self) -> bool {
        self.insts
            .last()
            .map(|v| v.inst.is_terminate_inst())
            .unwrap_or(false)
    }
}
