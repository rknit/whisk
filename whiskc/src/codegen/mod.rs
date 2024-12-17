use std::collections::HashMap;

use wsk_vm::program::{Function, Program};

use crate::{
    ast_resolved::{nodes::item::Item, ResolvedAST},
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
        Ok(ctx.prog)
    } else {
        Err(CodegenError::NoMainFunction)
    }
}

struct Context {
    pub prog: Program,
    fis: HashMap<SymbolID, usize>,
    cur_fi: Option<usize>,
    locals: Vec<Option<SymbolID>>,
    stack_bounds: Vec<usize>,
}
impl Context {
    pub fn new() -> Self {
        Self {
            prog: Program::new(),
            fis: HashMap::new(),
            cur_fi: None,
            locals: vec![],
            stack_bounds: vec![],
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

    pub fn push_bound(&mut self) {
        self.stack_bounds.push(self.locals.len());
    }

    pub fn pop_bound(&mut self) {
        let [.., bound] = self.stack_bounds[..] else {
            return;
        };
        while self.locals.len() != bound {
            self.locals.pop();
        }
    }

    pub fn push_local(&mut self, sym_id: Option<SymbolID>) {
        self.locals.push(sym_id);
    }

    pub fn pop_local(&mut self) {
        self.locals.pop().expect("local to pop");
    }

    pub fn get_local(&self, sym_id: SymbolID) -> Option<usize> {
        let index = self
            .locals
            .iter()
            .position(|v| matches!(v, Some(v) if *v == sym_id))?;
        Some(self.locals.len() - 1 - index)
    }

    pub fn clear_locals(&mut self) {
        self.locals.clear();
    }

    pub fn add_fi(&mut self, sym_id: SymbolID, fi: usize) {
        assert!(!self.fis.contains_key(&sym_id), "duplicate symbols to fi");
        self.fis.insert(sym_id, fi);
    }

    pub fn get_fi(&self, sym_id: SymbolID) -> Option<usize> {
        self.fis.get(&sym_id).copied()
    }
}

#[derive(Debug)]
pub enum CodegenError {
    UnsupportedItem,
    NoMainFunction,
}

trait Codegen {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError>;
}
