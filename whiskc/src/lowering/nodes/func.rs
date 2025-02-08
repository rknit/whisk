use crate::{old_symbol_table::SymbolID, symbol::FuncId};

use super::expr::BlockExpr;

#[derive(Debug, Clone)]
pub struct Function {
    pub sym_id: SymbolID,
    pub func_id: FuncId,
    pub body: BlockExpr,
}

#[derive(Debug, Clone)]
pub struct ExternFunction(pub SymbolID, pub FuncId);
