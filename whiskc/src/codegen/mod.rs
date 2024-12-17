use std::collections::HashMap;

use wsk_vm::program::{Function, Program};

use crate::{
    ast_resolved::{nodes::item::Item, ResolvedAST},
    symbol_table::{Symbol, SymbolID, SymbolTable},
};

mod expr;
mod func;
mod stmt;

pub fn codegen_wsk_vm(ast: &ResolvedAST, sym_table: &SymbolTable) -> Result<Program, CodegenError> {
    let mut ctx = Context::new(sym_table);

    for item in &ast.items {
        let Item::Function(func) = item else {
            return Err(CodegenError::UnsupportedItem);
        };
        let fi = ctx.prog.add_func(Function::new());
        ctx.add_fi(func.sig.sym_id, fi);
    }

    for item in &ast.items {
        let Item::Function(func) = item else {
            continue;
        };
        ctx.clear_locals();
        func.codegen(&mut ctx)?;
    }

    Ok(ctx.prog)
}

struct Context<'a> {
    pub sym_table: &'a SymbolTable,
    pub prog: Program,
    fis: HashMap<SymbolID, usize>,
    cur_fi: Option<usize>,
    locals: Vec<Option<SymbolID>>,
    stack_bounds: Vec<usize>,
}
impl<'a> Context<'a> {
    pub fn new(sym_table: &'a SymbolTable) -> Self {
        Self {
            sym_table,
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

    pub fn get_symbol(&self, table_id: SymbolID, sym_id: SymbolID) -> Option<&Symbol> {
        self.sym_table.get_table(table_id)?.get_symbol(sym_id)
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
}

trait Codegen {
    fn codegen(&self, ctx: &mut Context) -> Result<(), CodegenError>;
}
