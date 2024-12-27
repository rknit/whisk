use crate::ast::nodes::punctuate::Punctuated;
use crate::ast::parsing::{
    token::{Delimiter, TokenKind},
    ParseContext, ParseResult,
};

impl<T> Punctuated<T> {
    pub fn parse<F>(
        parser: &mut ParseContext,
        sep: Delimiter,
        delim: Delimiter,
        mut parse_fn: F,
    ) -> ParseResult<Self>
    where
        F: FnMut(&mut ParseContext) -> ParseResult<T>,
    {
        let mut items = Vec::new();
        while !matches!(parser.lexer.peek_token_kind(0), TokenKind::Delimiter(d) if *d == delim) {
            items.push(parse_fn(parser)?);

            if !matches!(parser.lexer.peek_token_kind(0), TokenKind::Delimiter(d) if *d == delim) {
                match_delimiter!(parser, sep =>);
            }
        }

        Some(Self { items, sep })
    }
}
