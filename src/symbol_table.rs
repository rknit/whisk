use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use crate::{
    ast::location::{Located, LocationRange},
    cfg::nodes::value::Value,
    ty::{FuncType, Type},
};

pub type SymbolID = uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    id: SymbolID,
    interned_id: HashMap<String, SymbolID>,
    entries: HashMap<SymbolID, TaggedSymbolTableEntry>,
}
impl SymbolTable {
    pub fn new() -> Self {
        Self {
            id: SymbolID::nil(),
            interned_id: HashMap::new(),
            entries: HashMap::new(),
        }
    }

    pub fn new_entry(&mut self, parent_id: SymbolID, mut entry: SymbolTableEntry) -> SymbolID {
        let id = self.new_id();
        entry.set_id(id);
        self.entries
            .insert(id, TaggedSymbolTableEntry { parent_id, entry });
        id
    }

    pub fn new_named_entry(
        &mut self,
        parent_id: SymbolID,
        name: &str,
        mut entry: SymbolTableEntry,
    ) -> Option<SymbolID> {
        if self.name_exists(name) {
            return None;
        }
        let id = self.intern(name.to_owned());
        entry.set_id(id);
        self.entries
            .insert(id, TaggedSymbolTableEntry { parent_id, entry });
        Some(id)
    }

    pub fn get_entry(&self, id: SymbolID) -> Option<&SymbolTableEntry> {
        self.entries.get(&id).map(|v| &v.entry)
    }

    pub fn get_entry_mut(&mut self, id: SymbolID) -> Option<&mut SymbolTableEntry> {
        self.entries.get_mut(&id).map(|v| &mut v.entry)
    }

    pub fn get_entry_parent_id(&self, id: SymbolID) -> Option<SymbolID> {
        self.entries.get(&id).map(|v| v.parent_id)
    }

    pub fn get_table(&self, id: SymbolID) -> Option<&SymbolTable> {
        match self.get_entry(id) {
            Some(SymbolTableEntry::Table(table)) => Some(table),
            _ => None,
        }
    }

    pub fn get_table_mut(&mut self, id: SymbolID) -> Option<&mut SymbolTable> {
        match self.get_entry_mut(id) {
            Some(SymbolTableEntry::Table(table)) => Some(table),
            _ => None,
        }
    }

    pub fn get_symbol(&self, id: SymbolID) -> Option<&Symbol> {
        match self.get_entry(id) {
            Some(SymbolTableEntry::Symbol(symbol)) => Some(symbol),
            _ => None,
        }
    }

    pub fn get_symbol_mut(&mut self, id: SymbolID) -> Option<&mut Symbol> {
        match self.get_entry_mut(id) {
            Some(SymbolTableEntry::Symbol(symbol)) => Some(symbol),
            _ => None,
        }
    }

    pub fn get_symbol_by_name(&self, name: &str) -> Option<&Symbol> {
        self.get_symbol(self.get_symbol_id_by_name(name)?)
    }

    pub fn get_symbol_by_name_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        self.get_symbol_mut(self.get_symbol_id_by_name(name)?)
    }

    pub fn exists(&self, id: SymbolID) -> bool {
        self.entries.contains_key(&id)
    }

    pub fn name_exists(&self, name: &str) -> bool {
        self.interned_id.contains_key(name)
    }

    pub fn get_symbol_id_by_name(&self, name: &str) -> Option<SymbolID> {
        self.interned_id.get(name).copied()
    }

    fn new_id(&mut self) -> SymbolID {
        loop {
            let id = Uuid::new_v4();
            if !self.entries.contains_key(&id) {
                return id;
            }
        }
    }

    fn intern(&mut self, name: String) -> SymbolID {
        let id = Uuid::new_v4();
        self.interned_id.insert(name, id);
        id
    }

    fn set_id(&mut self, id: SymbolID) {
        self.id = id;
    }
}

#[derive(Debug, Clone)]
struct TaggedSymbolTableEntry {
    pub parent_id: SymbolID,
    pub entry: SymbolTableEntry,
}

#[derive(Debug, Clone)]
pub enum SymbolTableEntry {
    Table(SymbolTable),
    Symbol(Symbol),
}
impl SymbolTableEntry {
    fn set_id(&mut self, id: SymbolID) {
        match self {
            SymbolTableEntry::Table(table) => table.set_id(id),
            SymbolTableEntry::Symbol(symbol) => symbol.set_id(id),
        }
    }
}
impl From<SymbolTable> for SymbolTableEntry {
    fn from(value: SymbolTable) -> Self {
        Self::Table(value)
    }
}
impl From<Symbol> for SymbolTableEntry {
    fn from(value: Symbol) -> Self {
        Self::Symbol(value)
    }
}
impl From<VarSymbol> for SymbolTableEntry {
    fn from(value: VarSymbol) -> Self {
        Self::Symbol(value.into())
    }
}
impl From<FuncSymbol> for SymbolTableEntry {
    fn from(value: FuncSymbol) -> Self {
        Self::Symbol(value.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Variable,
    Function,
}

#[derive(Debug, Clone)]
pub enum Symbol {
    Variable(VarSymbol),
    Function(FuncSymbol),
}
impl Symbol {
    pub fn get_name(&self) -> &str {
        match self {
            Symbol::Variable(symbol) => symbol.get_name(),
            Symbol::Function(symbol) => symbol.get_name(),
        }
    }

    fn set_id(&mut self, id: SymbolID) {
        match self {
            Symbol::Variable(symbol) => symbol.set_id(id),
            Symbol::Function(symbol) => symbol.set_id(id),
        }
    }

    pub fn get_id(&self) -> SymbolID {
        match self {
            Symbol::Variable(symbol) => symbol.get_id(),
            Symbol::Function(symbol) => symbol.get_id(),
        }
    }

    pub fn get_type(&self) -> Type {
        match self {
            Symbol::Variable(symbol) => symbol.get_type(),
            Symbol::Function(symbol) => symbol.get_type(),
        }
    }

    pub fn get_kind(&self) -> SymbolKind {
        match self {
            Symbol::Variable(_) => SymbolKind::Variable,
            Symbol::Function(_) => SymbolKind::Function,
        }
    }

    pub fn get_origin(&self) -> LocationRange {
        match self {
            Symbol::Variable(symbol) => symbol.get_origin(),
            Symbol::Function(symbol) => symbol.get_origin(),
        }
    }
}
impl From<VarSymbol> for Symbol {
    fn from(value: VarSymbol) -> Self {
        Self::Variable(value)
    }
}
impl From<FuncSymbol> for Symbol {
    fn from(value: FuncSymbol) -> Self {
        Self::Function(value)
    }
}

#[derive(Debug, Clone)]
pub struct VarSymbol {
    id: SymbolID,
    name: Located<String>,
    ty: Type,
    value: Option<Value>,
}
impl VarSymbol {
    pub fn new(name: Located<String>, ty: Type) -> Self {
        Self {
            id: SymbolID::nil(),
            name,
            ty,
            value: None,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name.0
    }

    fn set_id(&mut self, id: SymbolID) {
        self.id = id;
    }

    pub fn get_id(&self) -> SymbolID {
        self.id
    }

    pub fn get_type(&self) -> Type {
        self.ty
    }

    pub fn get_origin(&self) -> LocationRange {
        self.name.1
    }

    pub fn set_value(&mut self, value: Value) {
        self.value = Some(value);
    }

    pub fn get_value(&self) -> Option<Value> {
        self.value
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SymbolAttribute {
    Public,
}

#[derive(Debug, Clone)]
pub struct FuncSymbol {
    id: SymbolID,
    name: Located<String>,
    params: Vec<(Located<String>, Type)>,
    ret_ty: Type,
    attributes: HashSet<SymbolAttribute>,
}
impl FuncSymbol {
    pub fn new(name: Located<String>, params: Vec<(Located<String>, Type)>, ret_ty: Type) -> Self {
        Self {
            id: SymbolID::nil(),
            name,
            params,
            ret_ty,
            attributes: HashSet::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name.0
    }

    fn set_id(&mut self, id: SymbolID) {
        self.id = id;
    }

    pub fn get_id(&self) -> SymbolID {
        self.id
    }

    pub fn add_attributes(&mut self, attributes: Vec<SymbolAttribute>) {
        self.attributes.extend(attributes);
    }

    pub fn get_params(&self) -> &Vec<(Located<String>, Type)> {
        &self.params
    }

    pub fn get_return_type(&self) -> Type {
        self.ret_ty
    }

    pub fn get_type(&self) -> Type {
        FuncType(self.id).into()
    }

    pub fn get_origin(&self) -> LocationRange {
        self.name.1
    }
}
