#![allow(dead_code)]

mod common;
mod symbol_id;
mod symbol_table;
pub mod ty;

pub use symbol_id::*;
pub use symbol_table::SymbolTable;

use self::symbol_table::{Block, Function, Type, Variable};

pub struct TypeSymbol<'a> {
    table: &'a mut SymbolTable,
    id: TypeId,
}
impl<'a> TypeSymbol<'a> {
    fn new(table: &'a mut SymbolTable, id: TypeId) -> Self {
        Self { table, id }
    }

    fn get(&self) -> &Type {
        self.table.types.get(&self.id).unwrap()
    }

    fn get_mut(&mut self) -> &mut Type {
        self.table.types.get_mut(&self.id).unwrap()
    }

    pub fn get_name(&self) -> &str {
        &self.get().name
    }

    pub fn get_id(&self) -> TypeId {
        self.id
    }
}

pub struct FuncSymbol<'a> {
    table: &'a mut SymbolTable,
    id: FuncId,
}
impl<'a> FuncSymbol<'a> {
    fn new(table: &'a mut SymbolTable, id: FuncId) -> Self {
        Self { table, id }
    }

    fn get(&self) -> &Function {
        self.table.funcs.get(&self.id).unwrap()
    }

    fn get_mut(&mut self) -> &mut Function {
        self.table.funcs.get_mut(&self.id).unwrap()
    }

    pub fn get_name(&self) -> &str {
        &self.get().name
    }

    pub fn params(&self) -> &Vec<VarId> {
        &self.get().params
    }

    pub fn set_params(&mut self, params: impl Into<Vec<VarId>>) -> &mut Self {
        self.get_mut().params = params.into();
        self
    }

    pub fn set_return_type(&mut self, ty: TypeId) -> &mut Self {
        self.get_mut().ret_ty = ty;
        self
    }

    pub fn get_return_type(&self) -> TypeId {
        self.get().ret_ty
    }

    pub fn set_entry_block(&mut self, block: BlockId) -> &mut Self {
        self.get_mut().entry_block = block;
        self
    }

    pub fn get_entry_block(&self) -> BlockId {
        self.get().entry_block
    }

    pub fn get_id(&self) -> FuncId {
        self.id
    }
}

pub struct BlockSymbol<'a> {
    table: &'a mut SymbolTable,
    id: BlockId,
}
impl<'a> BlockSymbol<'a> {
    fn new(table: &'a mut SymbolTable, id: BlockId) -> Self {
        Self { table, id }
    }

    fn get(&self) -> &Block {
        self.table.blocks.get(&self.id).unwrap()
    }

    fn get_mut(&mut self) -> &mut Block {
        self.table.blocks.get_mut(&self.id).unwrap()
    }

    pub fn set_parent_block(&mut self, block: BlockId) -> &mut Self {
        assert!(
            self.id != block,
            "cannot assign the block itself as its parent block"
        );
        self.get_mut().parent_block = Some(block);
        self
    }

    pub fn get_parent_block(&self) -> Option<BlockId> {
        self.get().parent_block
    }

    pub fn get_function(&self) -> FuncId {
        self.get().func
    }

    pub fn get_id(&self) -> BlockId {
        self.id
    }
}

pub struct VarSymbol<'a> {
    table: &'a mut SymbolTable,
    id: VarId,
}
impl<'a> VarSymbol<'a> {
    fn new(table: &'a mut SymbolTable, id: VarId) -> Self {
        Self { table, id }
    }

    fn get(&self) -> &Variable {
        self.table.vars.get(&self.id).unwrap()
    }

    fn get_mut(&mut self) -> &mut Variable {
        self.table.vars.get_mut(&self.id).unwrap()
    }

    pub fn get_name(&self) -> &str {
        &self.get().name
    }

    pub fn set_type(&mut self, ty: TypeId) -> &mut Self {
        self.get_mut().ty = ty;
        self
    }

    pub fn get_type(&self) -> TypeId {
        self.get().ty
    }

    pub fn get_block(&self) -> BlockId {
        self.get().block
    }

    pub fn get_id(&self) -> VarId {
        self.id
    }
}
