use std::collections::HashMap;

use wsk_vm::{
    program::{Function, Program},
    Inst,
};

use crate::{
    module::{nodes::item::Item, Module},
    symbol::{FuncId, SymbolTable, VarId},
};

mod expr;
mod func;
mod stmt;

pub fn codegen_wsk_vm(module: &Module) -> Result<Program, CodegenError> {
    let mut ctx = Context::new(&module.sym_table);
    let mut has_entry = false;

    for item in &module.items {
        let Item::Function(func) = item else {
            return Err(CodegenError::UnsupportedItem);
        };
        let fi = ctx.prog.add_func(Function::default());
        ctx.add_fi(func.func_id, fi);

        let func_sym = func.func_id.sym(ctx.sym_table);
        if func_sym.name == "main" {
            ctx.prog.set_entry_point(fi);
            has_entry = true;

            if !func_sym.params.is_empty() || func_sym.ret_ty != ctx.sym_table.common_type().int {
                return Err(CodegenError::UnsupportedMainFunctionSig);
            }
        }
    }

    for item in &module.items {
        let Item::Function(func) = item else {
            continue;
        };
        ctx.clear_locals();
        func.codegen(&mut ctx)?;
    }

    if has_entry {
        // attach runtime entry
        let rtfunc = Function::from_insts([Inst::Call(ctx.prog.get_entry_point()), Inst::Halt]);
        let rtid = ctx.prog.add_func(rtfunc);
        ctx.prog.set_entry_point(rtid);

        Ok(ctx.prog)
    } else {
        Err(CodegenError::NoMainFunction)
    }
}

struct Context<'a> {
    pub sym_table: &'a SymbolTable,
    pub prog: Program,
    fis: HashMap<FuncId, usize>,
    cur_fi: Option<usize>,
    locals: HashMap<VarId, usize>,
    local_cnts: Vec<usize>,
    active_local_cnt: usize,
}
impl<'a> Context<'a> {
    pub fn new(sym_table: &'a SymbolTable) -> Self {
        Self {
            sym_table,
            prog: Program::default(),
            fis: HashMap::new(),
            cur_fi: None,
            locals: HashMap::new(),
            local_cnts: vec![],
            active_local_cnt: 0,
        }
    }

    pub fn set_current_fi(&mut self, fid: FuncId) {
        let fi = self.get_fi(fid).expect("set fi");
        self.cur_fi = Some(fi);
    }

    pub fn unset_current_fi(&mut self) {
        self.cur_fi = None;
    }

    pub fn get_current_fi_mut(&mut self) -> &mut Function {
        let fi = self.cur_fi.expect("within function context");
        self.prog.get_mut(fi).unwrap()
    }

    pub fn add_fi(&mut self, fid: FuncId, fi: usize) {
        assert!(!self.fis.contains_key(&fid), "duplicate symbols to fi");
        self.fis.insert(fid, fi);
    }

    pub fn get_fi(&self, fid: FuncId) -> Option<usize> {
        self.fis.get(&fid).copied()
    }

    pub fn clear_locals(&mut self) {
        self.locals.clear();
        self.local_cnts.clear();
        self.active_local_cnt = 0;
    }

    pub fn get_local(&mut self, vid: VarId) -> usize {
        if let Some(id) = self.locals.get(&vid) {
            return *id;
        }
        let id = self.active_local_cnt;
        self.active_local_cnt += 1;
        self.locals.insert(vid, id);
        id
    }

    pub fn push_bound(&mut self) {
        self.local_cnts.push(self.active_local_cnt);
    }

    pub fn pop_bound(&mut self) {
        let new_active = self.local_cnts.pop().expect("no bound to pop");
        self.active_local_cnt = new_active;
    }
}

#[derive(Debug)]
pub enum CodegenError {
    UnsupportedItem,
    NoMainFunction,
    UnsupportedMainFunctionSig,
}

trait Codegen {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError>;
}
