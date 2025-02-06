use crate::old_symbol_table::SymbolID;

use super::expr::BlockExpr;

#[derive(Debug, Clone)]
pub struct Function {
    pub sym_id: SymbolID,
    pub body: BlockExpr,
}

#[derive(Debug, Clone)]
pub struct ExternFunction(pub SymbolID);
