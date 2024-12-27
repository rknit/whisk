use crate::{
    ast::{
        location::Located,
        parsing::token::{Delimiter, Keyword},
    },
    ty::Type,
};

use super::{attributes::Attributes, expr::BlockExpr, punctuate::Punctuated};

#[derive(Debug, Clone)]
pub struct Function {
    pub sig: FunctionSig,
    pub body: BlockExpr,
}

#[derive(Debug, Clone)]
pub struct ExternFunction {
    pub extern_tok: Located<Keyword>,
    pub sig: FunctionSig,
    pub semi_tok: Located<Delimiter>,
}

#[derive(Debug, Clone)]
pub struct Param(pub Located<String>, pub Located<Type>);

#[derive(Debug, Clone)]
pub struct FunctionSig {
    pub attributes: Attributes,
    pub func_tok: Located<Keyword>,
    pub name: Located<String>,
    pub paren_open_tok: Located<Delimiter>,
    pub params: Punctuated<Param>,
    pub paren_close_tok: Located<Delimiter>,
    pub ret_ty: Located<Type>,
}
