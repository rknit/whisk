use crate::ast::location::Located;

#[derive(Debug, Clone)]
pub enum Expr {
    Constant(ConstantExpr),
    Ident(Located<String>),
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
impl From<Located<i64>> for ConstantExpr {
    fn from(value: Located<i64>) -> Self {
        Self::Integer(value)
    }
}
impl From<Located<i64>> for Expr {
    fn from(value: Located<i64>) -> Self {
        ConstantExpr::Integer(value).into()
    }
}
impl From<Located<bool>> for ConstantExpr {
    fn from(value: Located<bool>) -> Self {
        Self::Bool(value)
    }
}
impl From<Located<bool>> for Expr {
    fn from(value: Located<bool>) -> Self {
        ConstantExpr::Bool(value).into()
    }
}
