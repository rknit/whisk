use core::fmt;

use crate::{
    ast::{
        location::Located,
        parsing::token::{Delimiter, Keyword},
    },
    ty::Type,
};

use super::{attributes::Attributes, punctuate::Puntuated, stmt::Block};

#[derive(Debug, Clone)]
pub struct Function {
    pub sig: FunctionSig,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct ExternFunction {
    pub extern_tok: Located<Keyword>,
    pub sig: FunctionSig,
    pub semi_tok: Located<Delimiter>,
}

#[derive(Clone)]
pub struct LocatedParam(pub Located<String>, pub Located<Type>);
impl fmt::Debug for LocatedParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}", self.0, self.1)
    }
}

#[derive(Debug, Clone)]
pub struct FunctionSig {
    pub attributes: Attributes,
    pub func_tok: Located<Keyword>,
    pub name: Located<String>,
    pub paren_open_tok: Located<Delimiter>,
    pub params: Puntuated<LocatedParam>,
    pub paren_close_tok: Located<Delimiter>,
    pub ret_ty: Located<Type>,
}
