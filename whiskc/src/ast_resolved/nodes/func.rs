use crate::{ast::location::Located, symbol_table::SymbolID, ty::Type};

use super::stmt::Block;

#[derive(Debug, Clone)]
pub struct Function {
    pub table_id: SymbolID,
    pub sig: FunctionSig,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct ExternFunction(pub FunctionSig);

#[derive(Debug, Clone)]
pub struct FunctionSig {
    pub sym_id: SymbolID,
    pub name: Located<String>,
    pub params: Vec<(Located<String>, Type)>,
    pub ret_ty: Type,
}
