use errors::ResolveError;

use crate::symbol_table::{Symbol, SymbolID, SymbolTable};

mod compute;
pub mod errors;
pub mod nodes;
mod resolve;

pub use nodes::ast::ResolvedAST;
pub use resolve::ast::resolve;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ControlFlow {
    Flow,
    Return,
}

trait Resolve<R: Sized> {
    fn resolve(&self, ctx: &mut ResolveContext) -> Option<R>;
}

#[derive(Debug)]
struct ResolveContext<'a> {
    errors: Vec<ResolveError>,
    global_table: &'a mut SymbolTable,
    local_tables: Vec<SymbolID>,
    func_sym_id: SymbolID,
}
impl<'a> ResolveContext<'a> {
    pub fn new(global_table: &'a mut SymbolTable) -> Self {
        Self {
            errors: vec![],
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

    pub fn get_func_symbol_id(&self) -> SymbolID {
        if self.func_sym_id == SymbolID::nil() {
            panic!("unset function symbol id");
        }
        self.func_sym_id
    }

    pub fn push_local(&mut self) -> SymbolID {
        let table = SymbolTable::new();
        let current_id = self.get_current_table_id().unwrap_or(SymbolID::nil());
        let table_id = self.global_table.new_entry(current_id, table.into());
        self.local_tables.push(table_id);
        table_id
    }

    pub fn pop_local(&mut self) {
        self.local_tables
            .pop()
            .expect("not pop empty local scope stack");
    }

    pub fn new_symbol(&mut self, name: &str, symbol: Symbol) -> Option<SymbolID> {
        let parent_id = self.get_current_table_id().unwrap_or(SymbolID::nil());
        self.get_current_table_mut()
            .new_named_entry(parent_id, name, symbol.into())
    }

    pub fn get_current_table_id(&self) -> Option<SymbolID> {
        self.local_tables.last().copied()
    }

    pub fn get_current_table_mut(&mut self) -> &mut SymbolTable {
        if let Some(local_table_id) = self.local_tables.last() {
            self.get_table_mut(*local_table_id).unwrap()
        } else {
            self.global_table
        }
    }

    pub fn get_symbol(&self, id: SymbolID) -> Option<&Symbol> {
        for table_id in self.local_tables.iter().rev() {
            let table = self.get_table(*table_id).unwrap();
            if let Some(symbol) = table.get_symbol(id) {
                return Some(symbol);
            }
        }
        self.global_table.get_symbol(id)
    }

    pub fn get_symbol_id_by_name(&self, name: &str) -> Option<SymbolID> {
        for table_id in self.local_tables.iter().rev() {
            let table = self.get_table(*table_id).unwrap();
            if let Some(id) = table.get_symbol_id_by_name(name) {
                return Some(id);
            }
        }
        self.global_table.get_symbol_id_by_name(name)
    }

    pub fn _get_symbol_by_name(&self, name: &str) -> Option<&Symbol> {
        for table_id in self.local_tables.iter().rev() {
            let table = self.get_table(*table_id).unwrap();
            if let Some(symbol) = table.get_symbol_by_name(name) {
                return Some(symbol);
            }
        }
        self.global_table.get_symbol_by_name(name)
    }

    pub fn get_symbol_by_name_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        let mut sym_table_id = SymbolID::nil();
        for table_id in self.local_tables.iter().rev() {
            let table = self.get_table(*table_id).unwrap();
            if table.get_symbol_by_name(name).is_some() {
                sym_table_id = *table_id;
                break;
            }
        }
        if sym_table_id == SymbolID::nil() {
            self.global_table.get_symbol_by_name_mut(name)
        } else {
            self.get_table_mut(sym_table_id)
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

    pub fn push_error(&mut self, e: ResolveError) {
        self.errors.push(e);
    }

    pub fn finalize(self) -> Result<(), Vec<ResolveError>> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors)
        }
    }
}
