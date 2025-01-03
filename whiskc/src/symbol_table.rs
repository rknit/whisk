use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use crate::{
    ast::location::{Located, Span},
    ast_resolved::nodes::ty::Type,
};

pub type SymbolID = uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    id: SymbolID,
    interned_id: HashMap<String, SymbolID>,
    entries: HashMap<SymbolID, TaggedSymbolTableEntry>,
}
impl Default for SymbolTable {
    fn default() -> Self {
        Self {
            id: SymbolID::nil(),
            interned_id: HashMap::new(),
            entries: HashMap::new(),
        }
    }
}
impl SymbolTable {
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

    macro_utils::decl_get_symbol!(var, Variable, VarSymbol);
    macro_utils::decl_get_symbol!(func, Function, FuncSymbol);
    macro_utils::decl_get_symbol!(type, Type, TypeSymbol);

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

    pub fn get_symbol_id_by_name(&self, name: &str) -> Option<SymbolID> {
        self.interned_id.get(name).copied()
    }

    pub fn exists(&self, id: SymbolID) -> bool {
        self.entries.contains_key(&id)
    }

    pub fn name_exists(&self, name: &str) -> bool {
        self.interned_id.contains_key(name)
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
    Type,
}

#[derive(Debug, Clone)]
pub enum Symbol {
    Variable(VarSymbol),
    Function(FuncSymbol),
    Type(TypeSymbol),
}
impl Symbol {
    pub fn get_name(&self) -> &str {
        match self {
            Symbol::Variable(symbol) => symbol.get_name(),
            Symbol::Function(symbol) => symbol.get_name(),
            Symbol::Type(symbol) => symbol.get_name(),
        }
    }

    fn set_id(&mut self, id: SymbolID) {
        match self {
            Symbol::Variable(symbol) => symbol.set_id(id),
            Symbol::Function(symbol) => symbol.set_id(id),
            Symbol::Type(symbol) => symbol.set_id(id),
        }
    }

    pub fn get_id(&self) -> SymbolID {
        match self {
            Symbol::Variable(symbol) => symbol.get_id(),
            Symbol::Function(symbol) => symbol.get_id(),
            Symbol::Type(symbol) => symbol.get_id(),
        }
    }

    pub fn get_type(&self) -> Type {
        match self {
            Symbol::Variable(symbol) => symbol.get_type(),
            Symbol::Function(symbol) => symbol.get_type(),
            Symbol::Type(symbol) => symbol.get_type(),
        }
    }

    pub fn get_kind(&self) -> SymbolKind {
        match self {
            Symbol::Variable(_) => SymbolKind::Variable,
            Symbol::Function(_) => SymbolKind::Function,
            Symbol::Type(_) => SymbolKind::Type,
        }
    }

    pub fn get_origin(&self) -> Span {
        match self {
            Symbol::Variable(symbol) => symbol.get_origin(),
            Symbol::Function(symbol) => symbol.get_origin(),
            Symbol::Type(symbol) => symbol.get_origin(),
        }
    }

    pub fn push_ref(&mut self, loc: Span) {
        match self {
            Symbol::Variable(symbol) => symbol.push_ref(loc),
            Symbol::Function(symbol) => symbol.push_ref(loc),
            Symbol::Type(symbol) => symbol.push_ref(loc),
        }
    }

    pub fn get_refs(&self) -> &Vec<Span> {
        match self {
            Symbol::Variable(symbol) => symbol.get_refs(),
            Symbol::Function(symbol) => symbol.get_refs(),
            Symbol::Type(symbol) => symbol.get_refs(),
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
impl From<TypeSymbol> for Symbol {
    fn from(value: TypeSymbol) -> Self {
        Self::Type(value)
    }
}

#[derive(Debug, Clone)]
pub struct TypeSymbol {
    id: SymbolID,
    name: Located<String>,
    ty: Type,
    refs: Vec<Span>,
}
impl TypeSymbol {
    pub fn new(name: Located<String>, ty: Type) -> Self {
        Self {
            id: SymbolID::nil(),
            name,
            ty,
            refs: vec![],
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

    pub fn get_origin(&self) -> Span {
        self.name.1
    }

    pub fn push_ref(&mut self, loc: Span) {
        if let [.., last] = self.refs[..] {
            if last > loc {
                panic!("reference location out of order");
            }
            if last == loc {
                panic!("duplicate reference location");
            }
        }
        self.refs.push(loc);
    }

    pub fn get_refs(&self) -> &Vec<Span> {
        &self.refs
    }
}

#[derive(Debug, Clone)]
pub struct VarSymbol {
    id: SymbolID,
    name: Located<String>,
    ty: Type,
    //value: Option<Value>,
    refs: Vec<Span>,
}
impl VarSymbol {
    pub fn new(name: Located<String>, ty: Type) -> Self {
        Self {
            id: SymbolID::nil(),
            name,
            ty,
            //value: None,
            refs: vec![],
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

    pub fn get_origin(&self) -> Span {
        self.name.1
    }

    pub fn push_ref(&mut self, loc: Span) {
        if let [.., last] = self.refs[..] {
            if last > loc {
                panic!("reference location out of order");
            }
            if last == loc {
                panic!("duplicate reference location");
            }
        }
        self.refs.push(loc);
    }

    pub fn get_refs(&self) -> &Vec<Span> {
        &self.refs
    }

    //pub fn set_value(&mut self, value: Value) {
    //    self.value = Some(value);
    //}
    //
    //pub fn get_value(&self) -> Option<Value> {
    //    self.value
    //}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SymbolAttribute {
    Public,
}

#[derive(Debug, Clone)]
pub struct FuncSymbol {
    id: SymbolID,
    table_id: SymbolID,
    name: Located<String>,
    params: Vec<SymbolID>,
    ret_ty: Type,
    attributes: HashSet<SymbolAttribute>,
    refs: Vec<Span>,
}
impl FuncSymbol {
    pub fn new(
        table_id: SymbolID,
        name: Located<String>,
        params: Vec<SymbolID>,
        ret_ty: Type,
    ) -> Self {
        Self {
            id: SymbolID::nil(),
            table_id,
            name,
            params,
            ret_ty,
            attributes: HashSet::new(),
            refs: vec![],
        }
    }

    pub fn get_table_id(&self) -> SymbolID {
        self.table_id
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

    pub fn get_param_ids(&self) -> &Vec<SymbolID> {
        &self.params
    }

    pub fn get_return_type(&self) -> Type {
        self.ret_ty
    }

    pub fn get_type(&self) -> Type {
        Type::Func(self.get_id())
    }

    pub fn get_origin(&self) -> Span {
        self.name.1
    }

    pub fn push_ref(&mut self, loc: Span) {
        if let [.., last] = self.refs[..] {
            if last > loc {
                panic!("reference location out of order");
            }
            if last == loc {
                panic!("duplicate reference location");
            }
        }
        self.refs.push(loc);
    }

    pub fn get_refs(&self) -> &Vec<Span> {
        &self.refs
    }
}

mod macro_utils {
    #[macro_export]
    macro_rules! decl_get_symbol {
        ($name:ident, $kind:ident, $ty:ty) => {
            paste::item! {
                pub fn [< get_ $name _symbol>](&self, id: SymbolID) -> Option<&$ty> {
                    match self.get_symbol(id) {
                        Some(Symbol::$kind(v)) => Some(v),
                        _ => None,
                    }
                }

                pub fn [< get_ $name _symbol_mut >](&mut self, id: SymbolID) -> Option<&mut $ty> {
                    match self.get_symbol_mut(id) {
                        Some(Symbol::$kind(v)) => Some(v),
                        _ => None,
                    }
                }
            }
        };
    }

    pub use decl_get_symbol;
}
