use crate::{ast::parsing::token::Operator, ast_resolved::compute::EvalConstant, ty::Type};

#[derive(Debug, Clone)]
pub struct Expr {
    kind: ExprKind,
    ty: Type,
}
impl Expr {
    pub fn new(kind: impl Into<ExprKind>, ty: Type) -> Self {
        let kind: ExprKind = kind.into();
        let eval_kind = if let Some(k) = kind.eval_constant() {
            k
        } else {
            kind
        };
        Self {
            kind: eval_kind,
            ty,
        }
    }

    pub fn get_kind(&self) -> &ExprKind {
        &self.kind
    }

    pub fn get_type(&self) -> Type {
        self.ty
    }
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Integer(i64),
    Bool(bool),
    Identifier(String),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Call(CallExpr),
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub op: Operator,
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub op: Operator,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
}

impl From<i64> for ExprKind {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl From<bool> for ExprKind {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
impl From<String> for ExprKind {
    fn from(value: String) -> Self {
        Self::Identifier(value)
    }
}
impl From<UnaryExpr> for ExprKind {
    fn from(value: UnaryExpr) -> Self {
        Self::Unary(value)
    }
}
impl From<BinaryExpr> for ExprKind {
    fn from(value: BinaryExpr) -> Self {
        Self::Binary(value)
    }
}
impl From<CallExpr> for ExprKind {
    fn from(value: CallExpr) -> Self {
        Self::Call(value)
    }
}
