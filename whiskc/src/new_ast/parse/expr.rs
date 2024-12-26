use std::io::prelude::Read;

use crate::{
    ast::location::Located,
    new_ast::{
        nodes::expr::{ConstantExpr, Expr},
        parser::{Parse, Parser, TryParse},
    },
};

impl TryParse for Expr {
    type Output = Self;

    fn try_parse<R: Read>(parser: &mut Parser<R>) -> Option<Self::Output> {
        todo!()
    }
}

impl TryParse for ConstantExpr {
    type Output = Self;

    fn try_parse<R: Read>(parser: &mut Parser<R>) -> Option<Self::Output> {
        todo!()
    }
}

impl TryParse for i64 {
    type Output = Located<Self>;

    fn try_parse<R: Read>(parser: &mut Parser<R>) -> Option<Self::Output> {
        todo!()
    }
}

impl TryParse for bool {
    type Output = Located<Self>;

    fn try_parse<R: Read>(parser: &mut Parser<R>) -> Option<Self::Output> {
        todo!()
    }
}

impl TryParse for String {
    type Output = Located<String>;

    fn try_parse<R: Read>(parser: &mut Parser<R>) -> Option<Self::Output> {
        todo!()
    }
}

impl Parse for Expr {
    type Output = Self;

    fn parse<R: Read>(parser: &mut Parser<R>) -> Option<Self::Output> {
        todo!()
    }
}
