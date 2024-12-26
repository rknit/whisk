use crate::ast::location::Located;

#[derive(Debug, Clone)]
pub enum Expr {
    Constant(ConstantExpr),
}

#[derive(Debug, Clone, Copy)]
pub enum ConstantExpr {
    Integer(Located<i64>),
    Bool(Located<bool>),
}

impl From<ConstantExpr> for Expr {
    fn from(value: ConstantExpr) -> Self {
        Self::Constant(value)
    }
}
