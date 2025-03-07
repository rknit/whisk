use crate::symbol::FuncId;

use super::expr::BlockExpr;

#[derive(Debug, Clone)]
pub struct Function {
    pub func_id: FuncId,
    pub body: BlockExpr,
}

#[derive(Debug, Clone)]
pub struct ExternFunction(pub FuncId);
