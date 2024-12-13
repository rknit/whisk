use std::collections::HashMap;

use crate::ast::parsing::token::{Token, TokenKind};

use super::super::{ParseContext, ParseResult};

#[derive(Debug)]
pub struct LookUpParser<R> {
    lookup: HashMap<TokenKind, Handler<R>>,
}
impl<R> LookUpParser<R> {
    pub fn new(handlers: &impl Handlers<R>) -> Self {
        let mut parser = Self {
            lookup: HashMap::new(),
        };
        handlers.handlers(|tt, handler| {
            parser.lookup.insert(tt, handler);
        });
        parser
    }

    pub fn parse(&self, parser: &mut ParseContext) -> LookUpParseResult<R> {
        let Some(handler_fn) = self.lookup.get(parser.lexer.peek_token_kind(0)) else {
            return Err(LookUpParseError::NoHandlerFound(
                parser.lexer.peek_token(0).clone(),
            ));
        };
        handler_fn(self, parser).ok_or(LookUpParseError::ParseError)
    }
}

#[derive(Debug, Clone)]
pub enum LookUpParseError {
    NoHandlerFound(Token),
    ParseError,
}

pub type LookUpParseResult<T> = Result<T, LookUpParseError>;

pub type Handler<R> = fn(&LookUpParser<R>, &mut ParseContext) -> ParseResult<R>;

pub trait Handlers<R> {
    fn handlers<F>(&self, handler: F)
    where
        F: FnMut(TokenKind, Handler<R>);
}
