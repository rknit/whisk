use std::collections::HashMap;

use wsk_vm::{
    program::{Function, Program},
    Inst,
};

use crate::{
    ast_resolved::{
        nodes::{item::Item, ty::Type},
        ResolvedAST,
    },
    symbol_table::SymbolID,
};

mod expr;
mod func;
mod stmt;

pub fn codegen_wsk_vm(ast: &ResolvedAST) -> Result<Program, CodegenError> {
    let mut ctx = Context::new();
    let mut has_entry = false;

    for item in &ast.items {
        let Item::Function(func) = item else {
            return Err(CodegenError::UnsupportedItem);
        };
        let fi = ctx.prog.add_func(Function::new());
        ctx.add_fi(func.sig.sym_id, fi);

        if func.sig.name.0 == "main" {
            ctx.prog.set_entry_point(fi);
            has_entry = true;

            if !func.sig.params.is_empty() || func.sig.ret_ty != Type::Int {
                return Err(CodegenError::UnsupportedMainFunctionSig);
            }
        }
    }

    for item in &ast.items {
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

struct Context {
    pub prog: Program,
    fis: HashMap<SymbolID, usize>,
    cur_fi: Option<usize>,
    locals: HashMap<SymbolID, usize>,
    local_cnts: Vec<usize>,
    active_local_cnt: usize,
}
impl Context {
    pub fn new() -> Self {
        Self {
            prog: Program::new(),
            fis: HashMap::new(),
            cur_fi: None,
            locals: HashMap::new(),
            local_cnts: vec![],
            active_local_cnt: 0,
        }
    }

    pub fn set_current_fi(&mut self, sym_id: SymbolID) {
        let fi = self.get_fi(sym_id).expect("set fi");
        self.cur_fi = Some(fi);
    }

    pub fn unset_current_fi(&mut self) {
        self.cur_fi = None;
    }

    pub fn get_current_fi_mut(&mut self) -> &mut Function {
        let fi = self.cur_fi.expect("within function context");
        self.prog.get_mut(fi).unwrap()
    }

    pub fn add_fi(&mut self, sym_id: SymbolID, fi: usize) {
        assert!(!self.fis.contains_key(&sym_id), "duplicate symbols to fi");
        self.fis.insert(sym_id, fi);
    }

    pub fn get_fi(&self, sym_id: SymbolID) -> Option<usize> {
        self.fis.get(&sym_id).copied()
    }

    pub fn clear_locals(&mut self) {
        self.locals.clear();
        self.local_cnts.clear();
        self.active_local_cnt = 0;
    }

    pub fn get_local(&mut self, sym_id: SymbolID) -> usize {
        if let Some(id) = self.locals.get(&sym_id) {
            return *id;
        }
        let id = self.active_local_cnt;
        self.active_local_cnt += 1;
        self.locals.insert(sym_id, id);
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
