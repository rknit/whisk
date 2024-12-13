use crate::symbol_table::{Symbol, SymbolID, SymbolTable};

use super::nodes::{
    basic_block::{BasicBlock, BasicBlockID},
    function::Function,
    inst::{BranchCondInst, BranchInst, Inst, InstID, InstKind},
};

pub struct Builder<'a> {
    func: &'a mut Function,
    current: BasicBlockID,
}
impl<'a> Builder<'a> {
    pub fn new(func: &'a mut Function, bb: BasicBlockID) -> Self {
        debug_assert!(
            func.is_valid_block(bb),
            "This Basic Block doesn't belong to the function"
        );
        Self { func, current: bb }
    }

    pub fn add_block(&mut self, desc: Option<String>) -> BasicBlockID {
        self.func.add_block(BasicBlock::new(desc))
    }

    pub fn get_block(&self, bb: BasicBlockID) -> Option<&BasicBlock> {
        self.func.get_block(bb)
    }

    fn get_block_mut(&mut self, bb: BasicBlockID) -> Option<&mut BasicBlock> {
        self.func.get_block_mut(bb)
    }

    pub fn set_current_block(&mut self, bb: BasicBlockID) {
        debug_assert!(
            self.func.is_valid_block(bb),
            "This Basic Block doesn't belong to the function"
        );
        self.current = bb;
    }

    pub fn get_current_block_id(&self) -> &BasicBlockID {
        &self.current
    }

    pub fn get_current_block(&self) -> &BasicBlock {
        self.func.get_block(self.current).unwrap()
    }

    pub fn is_termiated(&self) -> bool {
        self.get_current_block().is_terminated()
    }

    /// Insert *inst* before *before_inst* at *bb* block.
    /// This operation will split the block if *inst* is a termiator instruction. (See
    /// cfg::nodes::function::Function::split_block_at for the split's implementation)
    /// Note: Current block will not be changed.
    fn insert_inst_at(
        &mut self,
        bb: BasicBlockID,
        inst: Inst,
        before_inst: Option<InstID>,
    ) -> Option<InstID> {
        let inst_id = self
            .get_block_mut(bb)?
            .insert_inst(inst.clone(), before_inst)?;

        // return early if push_inst
        if before_inst.is_none() {
            //debug_assert!(
            //    !self.get_block(bb).unwrap().is_terminated(),
            //    "pushing inst into a terminated block"
            //);
            return Some(inst_id);
        }

        match &inst.kind {
            InstKind::Branch(BranchInst { branch }) => {
                let (bb1, _) = self.func.split_block_at(bb, inst_id)?;
                self.func.link_block(bb1, *branch);
            }
            InstKind::BranchCond(BranchCondInst {
                then_branch,
                else_branch,
                ..
            }) => {
                let (bb1, _) = self.func.split_block_at(bb, inst_id)?;
                self.func.link_block(bb1, *then_branch);
                self.func.link_block(bb1, *else_branch);
            }
            _ => (),
        };
        Some(inst_id)
    }

    pub fn insert_inst(&mut self, inst: Inst, before_inst: InstID) -> Option<InstID> {
        self.insert_inst_at(self.current, inst, Some(before_inst))
    }

    pub fn push_inst(&mut self, inst: Inst) -> InstID {
        self.insert_inst_at(self.current, inst, None).unwrap()
    }
}

pub(super) trait BuildVisitor<R> {
    fn visit(&self, ctx: &mut BuildContext, builder: &mut Builder) -> R;
}

#[derive(Debug)]
pub(super) struct BuildContext<'a> {
    global_table: &'a mut SymbolTable,
    local_tables: Vec<SymbolID>,
    func_sym_id: SymbolID,
}
impl<'a> BuildContext<'a> {
    pub fn new(global_table: &'a mut SymbolTable) -> Self {
        Self {
            global_table,
            local_tables: vec![],
            func_sym_id: SymbolID::nil(),
        }
    }

    pub fn set_func_symbol_id(&mut self, id: SymbolID) {
        assert!(
            self.func_sym_id == SymbolID::nil(),
            "no function set before setting the function"
        );
        self.func_sym_id = id;
    }

    pub fn unset_func_symbol_id(&mut self) {
        assert!(
            self.local_tables.is_empty(),
            "all local tables must be popped before unsetting the function"
        );
        self.func_sym_id = SymbolID::nil();
    }

    pub fn push_local(&mut self, table_id: SymbolID) {
        self.local_tables.push(table_id);
    }

    pub fn pop_local(&mut self) {
        self.local_tables
            .pop()
            .expect("not pop empty local scope stack");
    }

    pub fn get_symbol_by_name(&self, name: &str) -> Option<&Symbol> {
        for table_id in self.local_tables.iter().rev() {
            let table = self.get_table(*table_id).unwrap();
            if let Some(symbol) = table.get_symbol_by_name(name) {
                return Some(symbol);
            }
        }
        self.global_table.get_symbol_by_name(name)
    }

    pub fn get_symbol_by_name_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        let mut exist_table_id = SymbolID::nil();
        for i in (0..self.local_tables.len()).rev() {
            let table_id = self.local_tables[i];
            let table = self.get_table_mut(table_id).unwrap();
            if table.name_exists(name) {
                exist_table_id = table_id;
                break;
            }
        }
        if exist_table_id == SymbolID::nil() {
            self.global_table.get_symbol_by_name_mut(name)
        } else {
            self.get_table_mut(exist_table_id)
                .unwrap()
                .get_symbol_by_name_mut(name)
        }
    }

    fn get_table(&self, table_id: SymbolID) -> Option<&SymbolTable> {
        self.global_table.get_table(table_id)
    }

    fn get_table_mut(&mut self, table_id: SymbolID) -> Option<&mut SymbolTable> {
        self.global_table.get_table_mut(table_id)
    }
}
