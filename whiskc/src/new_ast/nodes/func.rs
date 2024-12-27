use crate::{
    ast::location::Located,
    new_ast::token::{Delimiter, Keyword},
    ty::Type,
};

#[derive(Debug, Clone)]
pub struct LocatedParam(pub Located<String>, pub Located<Type>);

#[derive(Debug, Clone)]
pub struct Function {
    pub func_tok: Located<Keyword>,
    pub name: Located<String>,
    pub paren_open_tok: Located<Delimiter>,
    // pub params: Puntuated<LocatedParam>,
    pub paren_close_tok: Located<Delimiter>,
    // pub ret_ty: Located<Type>,
    // pub block: BlockExpr.
}
