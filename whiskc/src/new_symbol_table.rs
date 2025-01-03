use std::collections::HashMap;

use crate::ast_resolved::nodes::ty::Type;

pub type SymbolID = uuid::Uuid;

#[derive(Debug, Default)]
pub struct SymbolTable {
    table: HashMap<SymbolID, Symbol>,
}
impl SymbolTable {
    pub fn new_symbol(&mut self, symbol: impl Into<Symbol>) -> SymbolID {
        let id = uuid::Uuid::new_v4();
        let sym: Symbol = symbol.into();
        sym.verify_symbol(self, id);
        self.table.insert(id, sym);
        id
    }

    pub fn get_scope(&self, sym_id: SymbolID) -> Option<&ScopeSymbol> {
        self.get_symbol(sym_id).and_then(|v| match v {
            Symbol::Scope(v) => Some(v),
            _ => None,
        })
    }
    pub fn get_scope_mut(&mut self, sym_id: SymbolID) -> Option<&mut ScopeSymbol> {
        self.get_symbol_mut(sym_id).and_then(|v| match v {
            Symbol::Scope(v) => Some(v),
            _ => None,
        })
    }

    pub fn get_var(&self, sym_id: SymbolID) -> Option<&VarSymbol> {
        self.get_symbol(sym_id).and_then(|v| match v {
            Symbol::Var(v) => Some(v),
            _ => None,
        })
    }
    pub fn get_var_mut(&mut self, sym_id: SymbolID) -> Option<&mut VarSymbol> {
        self.get_symbol_mut(sym_id).and_then(|v| match v {
            Symbol::Var(v) => Some(v),
            _ => None,
        })
    }

    pub fn get_func(&self, sym_id: SymbolID) -> Option<&FuncSymbol> {
        self.get_symbol(sym_id).and_then(|v| match v {
            Symbol::Func(v) => Some(v),
            _ => None,
        })
    }
    pub fn get_func_mut(&mut self, sym_id: SymbolID) -> Option<&mut FuncSymbol> {
        self.get_symbol_mut(sym_id).and_then(|v| match v {
            Symbol::Func(v) => Some(v),
            _ => None,
        })
    }

    pub fn get_symbol(&self, sym_id: SymbolID) -> Option<&Symbol> {
        self.table.get(&sym_id)
    }
    pub fn get_symbol_mut(&mut self, sym_id: SymbolID) -> Option<&mut Symbol> {
        self.table.get_mut(&sym_id)
    }
}

trait VerifySymbol {
    fn verify_symbol(&self, sym_table: &mut SymbolTable, self_id: SymbolID);
}

#[derive(Debug)]
pub enum Symbol {
    Scope(ScopeSymbol),
    Var(VarSymbol),
    Func(FuncSymbol),
    Type(TypeSymbol),
}
impl VerifySymbol for Symbol {
    fn verify_symbol(&self, sym_table: &mut SymbolTable, self_id: SymbolID) {
        match self {
            Symbol::Scope(v) => v.verify_symbol(sym_table, self_id),
            Symbol::Var(v) => v.verify_symbol(sym_table, self_id),
            Symbol::Func(v) => v.verify_symbol(sym_table, self_id),
            Symbol::Type(v) => v.verify_symbol(sym_table, self_id),
        }
    }
}

#[derive(Debug)]
pub struct ScopeSymbol {
    parent_id: SymbolID,
    children: Vec<SymbolID>,
}
impl ScopeSymbol {
    pub fn new(parent_id: SymbolID) -> Self {
        Self {
            parent_id,
            children: vec![],
        }
    }

    fn add_child(&mut self, sym_id: SymbolID) {
        self.children.push(sym_id);
    }
}
impl VerifySymbol for ScopeSymbol {
    fn verify_symbol(&self, sym_table: &mut SymbolTable, self_id: SymbolID) {
        if let Some(scope) = sym_table.get_scope_mut(self.parent_id) {
            scope.add_child(self_id);
        } else if sym_table.get_func(self.parent_id).is_some() {
        } else {
            panic!("parent of a scope must be an another scope or a function");
        }
    }
}

#[derive(Debug)]
pub struct VarSymbol {
    parent_id: SymbolID, // must be scope symbol id
    name: String,
    ty: Type,
    refs: usize,
}
impl VarSymbol {
    pub fn new(parent_id: SymbolID, name: String, ty: Type) -> Self {
        Self {
            parent_id,
            name,
            ty,
            refs: 0,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_type(&self) -> Type {
        self.ty
    }

    pub fn add_refs(&mut self, delta: isize) {
        self.refs = self
            .refs
            .checked_add_signed(delta)
            .expect("adding resulting in overflow/underflow");
    }

    pub fn get_refs(&self) -> usize {
        self.refs
    }
}
impl VerifySymbol for VarSymbol {
    fn verify_symbol(&self, sym_table: &mut SymbolTable, self_id: SymbolID) {
        if let Some(scope) = sym_table.get_scope_mut(self.parent_id) {
            scope.add_child(self_id);
        } else {
            panic!("parent of a variable must be a scope");
        }
    }
}

#[derive(Debug)]
pub struct FuncSymbol {
    table_id: SymbolID,
    name: String,
    params: Vec<SymbolID>,
    ret_ty: Type,
    refs: usize,
}
impl FuncSymbol {
    pub fn new(table_id: SymbolID, name: String, params: Vec<SymbolID>, ret_ty: Type) -> Self {
        Self {
            table_id,
            name,
            params,
            ret_ty,
            refs: 0,
        }
    }

    pub fn get_table(&self) -> SymbolID {
        self.table_id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_params(&self) -> &Vec<SymbolID> {
        &self.params
    }

    pub fn get_ret_type(&self) -> Type {
        self.ret_ty
    }

    pub fn add_refs(&mut self, delta: isize) {
        self.refs = self
            .refs
            .checked_add_signed(delta)
            .expect("adding resulting in overflow/underflow");
    }

    pub fn get_refs(&self) -> usize {
        self.refs
    }
}
impl VerifySymbol for FuncSymbol {
    fn verify_symbol(&self, _sym_table: &mut SymbolTable, _self_id: SymbolID) {}
}

#[derive(Debug)]
pub struct TypeSymbol {}
impl VerifySymbol for TypeSymbol {
    fn verify_symbol(&self, _sym_table: &mut SymbolTable, _self_id: SymbolID) {}
}
