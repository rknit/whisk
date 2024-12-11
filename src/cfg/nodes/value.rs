use crate::{symbol_table::SymbolID, ty::Type};

use super::{basic_block::BasicBlockID, inst::InstID};

#[derive(Debug, Clone, Copy)]
pub struct Value {
    pub kind: ValueKind,
    pub ty: Type,
}

#[derive(Debug, Clone, Copy)]
pub enum ValueKind {
    Constant(ConstantValue),
    Inst(InstValue),
    Parameter(SymbolID),
    Function(SymbolID),
}

#[derive(Debug, Clone, Copy)]
pub struct InstValue {
    pub bb: BasicBlockID,
    pub inst: InstID,
}

#[derive(Debug, Clone, Copy)]
pub enum ConstantValue {
    Bool(bool),
    Integer(i64),
}
