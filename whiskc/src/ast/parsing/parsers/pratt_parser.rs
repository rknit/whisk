use core::fmt;
use std::collections::HashMap;

use crate::ast::parsing::{
    token::{Token, TokenKind},
    ParseContext, ParseResult,
};

#[derive(Debug)]
pub struct PrattParser<R, BP> {
    nud_lookup: HashMap<TokenKind, NudHandler<R, BP>>,
    led_lookup: HashMap<TokenKind, LedHandler<R, BP>>,
    bp_lookup: HashMap<TokenKind, BP>,
}
impl<R, BP: PartialOrd + BindingPower + Clone + fmt::Debug> PrattParser<R, BP> {
    pub fn new(handlers: &impl Handlers<R, BP>) -> Self {
        let mut parser = Self {
            nud_lookup: HashMap::new(),
            led_lookup: HashMap::new(),
            bp_lookup: HashMap::new(),
        };
        handlers.nuds(|tt, handler| parser.nud(tt, handler));
        handlers.leds(|tt, bp, handler| parser.led(tt, bp, handler));
        parser
    }

    fn nud(&mut self, tt: TokenKind, handler: NudHandler<R, BP>) {
        self.nud_lookup.insert(tt, handler);
    }

    fn led(&mut self, tt: TokenKind, bp: BP, handler: LedHandler<R, BP>) {
        self.led_lookup.insert(tt.clone(), handler);
        self.bp_lookup.insert(tt, bp);
    }

    pub fn parse(&self, parser: &mut ParseContext, bp: BP) -> PrattParseResult<R> {
        let Some(nud_fn) = self.nud_lookup.get(parser.lexer.peek_token_kind(0)) else {
            return Err(PrattParseError::NoNudHandlerFound(
                parser.lexer.peek_token(0).clone(),
            ));
        };
        let mut left = nud_fn(self, parser).ok_or(PrattParseError::ParseError)?;

        loop {
            let tt = parser.lexer.peek_token_kind(0);

            let cur_bp = if let Some(bp) = self.bp_lookup.get(tt) {
                bp
            } else {
                &BP::zero()
            };

            if *cur_bp <= bp {
                break;
            }

            let Some(led_fn) = self.led_lookup.get(tt) else {
                return Err(PrattParseError::NoLedHandlerFound(
                    parser.lexer.peek_token(0).clone(),
                ));
            };
            left = led_fn(self, parser, left, cur_bp.clone()).ok_or(PrattParseError::ParseError)?;
        }

        Ok(left)
    }
}

#[derive(Debug, Clone)]
pub enum PrattParseError {
    NoNudHandlerFound(Token),
    NoLedHandlerFound(Token),
    ParseError,
}

pub type PrattParseResult<T> = Result<T, PrattParseError>;

pub type NudHandler<R, BP> = fn(&PrattParser<R, BP>, &mut ParseContext) -> ParseResult<R>;
pub type LedHandler<R, BP> = fn(&PrattParser<R, BP>, &mut ParseContext, R, BP) -> ParseResult<R>;

pub trait Handlers<R, BP> {
    fn nuds<F>(&self, nud: F)
    where
        F: FnMut(TokenKind, NudHandler<R, BP>);

    fn leds<F>(&self, led: F)
    where
        F: FnMut(TokenKind, BP, LedHandler<R, BP>);
}

pub trait BindingPower {
    fn primary() -> Self;
    fn zero() -> Self;
}
