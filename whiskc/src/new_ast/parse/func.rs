use std::io::prelude::Read;

use crate::{
    match_delim, match_identifier, match_keyword,
    new_ast::{
        errors::ParseErrorReport,
        nodes::func::Function,
        parser::Parser,
        token::{Delimiter, Keyword},
    },
};

use super::Parse;

impl Parse for Function {
    type Output = Self;

    fn parse<R: Read>(parser: &mut Parser<R>) -> Result<Self::Output, ParseErrorReport> {
        let mut rep = ParseErrorReport::default();
        let func_tok = match_keyword!(parser, Keyword::Func =>, rep);
        let func_name = match_identifier!(parser => "function name", rep);
        let paren_open_tok = match_delim!(parser, Delimiter::ParenOpen =>, rep);
        let paren_close_tok = match_delim!(parser, Delimiter::ParenClose =>, rep);

        if rep.is_ok() {
            Ok(Function {
                func_tok,
                name: func_name,
                paren_open_tok,
                paren_close_tok,
            })
        } else {
            Err(rep)
        }
    }
}
