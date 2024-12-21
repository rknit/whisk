use crate::{ast::location::Located, symbol_table::SymbolID, ty::Type};

use super::expr::BlockExpr;

#[derive(Debug, Clone)]
pub struct Function {
    pub table_id: SymbolID,
    pub sig: FunctionSig,
    pub body: BlockExpr,
}

#[derive(Debug, Clone)]
pub struct ExternFunction(pub FunctionSig);

#[derive(Debug, Clone)]
pub struct FunctionSig {
    pub sym_id: SymbolID,
    pub name: Located<String>,
    pub params: Vec<Param>,
    pub ret_ty: Type,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub sym_id: SymbolID,
    pub name: String,
}
