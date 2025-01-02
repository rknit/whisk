use once_cell::sync::Lazy;
use strum::IntoEnumIterator;

use crate::ast::{
    location::{Located, Span},
    nodes::ty::{PrimType, Type},
    parsing::{
        parsers::pratt_parser::{self, PrattParseError, PrattParser},
        token::{Delimiter, Identifier, Token, TokenKind, TypeKeyword},
        Parse, ParseContext, ParseError, ParseResult, TryParse,
    },
};

#[derive(Debug, Clone)]
pub enum TypeParseError {
    UnexpectedToken(TokenKind),
    UnexpectedInfixOperator(TokenKind),
    IntegerSizeOutOfRange(u16),
    ExpectedArrayLength,
    InvalidArrayLength(i64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum BindingPower {
    Zero,
    Primary,
}

struct TypeHandlers;
impl pratt_parser::Handlers<Type, BindingPower> for TypeHandlers {
    fn nuds<F>(&self, mut nud: F)
    where
        F: FnMut(TokenKind, pratt_parser::NudHandler<Type, BindingPower>),
    {
        nud(
            TokenKind::Identifier(Identifier("".to_owned())),
            |_, parser| parse_ident_type(parser),
        );

        for kw in TypeKeyword::iter() {
            nud(TokenKind::TypeKeyword(kw), |_, parser| {
                parse_keyword_type(parser)
            });
        }

        nud(TokenKind::Delimiter(Delimiter::ParenOpen), |_, parser| {
            parse_unit_type(parser)
        });
    }

    fn leds<F>(&self, _led: F)
    where
        F: FnMut(TokenKind, BindingPower, pratt_parser::LedHandler<Type, BindingPower>),
    {
    }
}

fn parse_unit_type(parser: &mut ParseContext) -> ParseResult<Type> {
    let paren_open_tok = match_delimiter!(parser, Delimiter::ParenOpen =>);
    let paren_close_tok = match_delimiter!(parser, Delimiter::ParenClose =>);
    Some(Type::Primitive(Located(
        PrimType::Unit,
        Span::combine(paren_open_tok.1, paren_close_tok.1),
    )))
}

fn parse_keyword_type(parser: &mut ParseContext) -> ParseResult<Type> {
    let Token {
        kind: TokenKind::TypeKeyword(kw),
        loc,
    } = parser.lexer.next_token()
    else {
        panic!("token should be a type keyword");
    };

    let prim_ty = match kw {
        TypeKeyword::Bool => PrimType::Bool,
        TypeKeyword::Int => PrimType::Int,
    };

    Some(Type::Primitive(Located(prim_ty, loc)))
}

fn parse_ident_type(parser: &mut ParseContext) -> ParseResult<Type> {
    let Token {
        kind: TokenKind::Identifier(Identifier(ident)),
        loc,
    } = parser.lexer.next_token()
    else {
        panic!("token should be an identifier");
    };

    Some(Type::Ident(Located(ident, loc)))
}

static PARSER: Lazy<PrattParser<Type, BindingPower>> =
    Lazy::new(|| PrattParser::<Type, BindingPower>::new(&TypeHandlers));

impl Parse for Type {
    fn parse(parser: &mut ParseContext) -> ParseResult<Self> {
        match PARSER.parse(parser, BindingPower::Zero) {
            Ok(v) => Some(v),
            Err(e) => {
                match e {
                    PrattParseError::NoNudHandlerFound(t) => parser.push_error(Located(
                        ParseError::TypeParseError(TypeParseError::UnexpectedToken(t.kind)),
                        t.loc,
                    )),
                    PrattParseError::NoLedHandlerFound(t) => parser.push_error(Located(
                        ParseError::TypeParseError(TypeParseError::UnexpectedInfixOperator(t.kind)),
                        t.loc,
                    )),
                    PrattParseError::ParseError => (),
                }
                None
            }
        }
    }
}
impl TryParse for Type {
    fn try_parse(ctx: &mut ParseContext) -> crate::ast::parsing::TryParseResult<Self> {
        PARSER.parse(ctx, BindingPower::Zero).ok()
    }
}

impl pratt_parser::BindingPower for BindingPower {
    fn primary() -> Self {
        Self::Primary
    }

    fn zero() -> Self {
        Self::Zero
    }
}
