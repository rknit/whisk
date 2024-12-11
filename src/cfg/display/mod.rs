use std::{collections::HashMap, fmt::Write};

use crate::symbol_table::{SymbolID, SymbolTable};

use super::{
    nodes::{basic_block::BasicBlockID, inst::InstID},
    CFG,
};

mod basic_block;
mod func;
mod inst;
mod value;

#[derive(Debug)]
struct DisplayContext<'a> {
    pub sym_table: &'a SymbolTable,
    reg_syms: HashMap<SymbolID, usize>,
    reg_insts: HashMap<InstID, usize>,
    reg_cnt: usize,
    bbs: HashMap<BasicBlockID, usize>,
    bb_cnt: usize,
}
impl<'a> DisplayContext<'a> {
    pub fn new(sym_table: &'a SymbolTable) -> Self {
        Self {
            sym_table,
            reg_syms: HashMap::new(),
            reg_insts: HashMap::new(),
            reg_cnt: 0,
            bbs: HashMap::new(),
            bb_cnt: 0,
        }
    }

    pub fn get_bb(&mut self, id: BasicBlockID) -> usize {
        if let Some(bb) = self.bbs.get(&id) {
            *bb
        } else {
            let bb = self.bb_cnt;
            self.bb_cnt += 1;
            self.bbs.insert(id, bb);
            bb
        }
    }

    pub fn get_reg_sym_id(&mut self, id: SymbolID) -> usize {
        if let Some(reg) = self.reg_syms.get(&id) {
            *reg
        } else {
            let reg = self.reg_cnt;
            self.reg_cnt += 1;
            self.reg_syms.insert(id, reg);
            reg
        }
    }

    pub fn get_reg_inst_id(&mut self, id: SymbolID) -> usize {
        if let Some(reg) = self.reg_insts.get(&id) {
            *reg
        } else {
            let reg = self.reg_cnt;
            self.reg_cnt += 1;
            self.reg_insts.insert(id, reg);
            reg
        }
    }
}

trait DisplayCFG {
    fn display<W: Write>(&self, w: &mut W, ctx: &mut DisplayContext);
}

pub fn display_cfg<W: Write>(w: &mut W, cfg: &CFG, sym_table: &SymbolTable) {
    for func in &cfg.funcs {
        let mut ctx = DisplayContext::new(sym_table);
        func.display(w, &mut ctx);
        writeln!(w, "").unwrap();
    }
}
