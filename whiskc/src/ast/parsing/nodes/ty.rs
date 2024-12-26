use std::collections::HashSet;

use once_cell::sync::Lazy;
use strum::IntoEnumIterator;

use crate::{
    ast::{
        location::{Located, Span},
        parsing::{
            parsers::pratt_parser::{self, PrattParseError, PrattParser},
            token::{Delimiter, Identifier, Literal, Token, TokenKind, TypeKeyword},
            Parse, ParseContext, ParseError, ParseResult, TryParse,
        },
    },
    ty::{PrimType, Type},
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
impl pratt_parser::Handlers<Located<Type>, BindingPower> for TypeHandlers {
    fn nuds<F>(&self, mut nud: F)
    where
        F: FnMut(TokenKind, pratt_parser::NudHandler<Located<Type>, BindingPower>),
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

        nud(TokenKind::Delimiter(Delimiter::BracketOpen), |_, parser| {
            parse_array_type(parser)
        });

        nud(TokenKind::Delimiter(Delimiter::ParenOpen), |_, parser| {
            parse_unit_type(parser)
        });
    }

    fn leds<F>(&self, _led: F)
    where
        F: FnMut(TokenKind, BindingPower, pratt_parser::LedHandler<Located<Type>, BindingPower>),
    {
    }

    fn delimiters(&self) -> HashSet<TokenKind> {
        HashSet::new()
    }
}

fn parse_unit_type(parser: &mut ParseContext) -> ParseResult<Located<Type>> {
    let paren_open_tok = match_delimiter!(parser, Delimiter::ParenOpen =>);
    let paren_close_tok = match_delimiter!(parser, Delimiter::ParenClose =>);
    Some(Located(
        PrimType::Unit.into(),
        Span::combine(paren_open_tok.1, paren_close_tok.1),
    ))
}

fn parse_array_type(parser: &mut ParseContext) -> ParseResult<Located<Type>> {
    let _brace_open_tok = match_delimiter!(parser, Delimiter::BracketOpen =>);
    let _el_ty = Located::<Type>::parse(parser)?;
    let _length = if matches!(
        parser.lexer.peek_token_kind(0),
        TokenKind::Delimiter(Delimiter::Colon)
    ) {
        let colon_tok = match_delimiter!(parser, Delimiter::Colon =>);
        let TokenKind::Literal(Literal::Int(v)) = parser.lexer.peek_token_kind(0) else {
            parser.push_error(Located(
                ParseError::TypeParseError(TypeParseError::ExpectedArrayLength),
                colon_tok.1.next().into(),
            ));
            return None;
        };
        let v = *v;
        let int_tok = parser.lexer.next_token();
        if v <= 0 {
            parser.push_error(Located(
                ParseError::TypeParseError(TypeParseError::InvalidArrayLength(v)),
                int_tok.loc,
            ));
            return None;
        }
        v as usize
    } else {
        0
    };

    #[allow(unreachable_code, unused_variables)]
    {
        let brace_close_tok = match_delimiter!(parser, Delimiter::BracketClose =>);
        Some(Located(
            todo!("array type parse"),
            Span::combine(_brace_open_tok.1, brace_close_tok.1),
        ))
    }
}

fn parse_keyword_type(parser: &mut ParseContext) -> ParseResult<Located<Type>> {
    let Token {
        kind: TokenKind::TypeKeyword(kw),
        loc,
    } = parser.lexer.next_token()
    else {
        panic!("token should be a type keyword");
    };

    Some(Located(
        match kw {
            TypeKeyword::Bool => PrimType::Bool,
            TypeKeyword::Int => PrimType::Integer,
        }
        .into(),
        loc,
    ))
}

fn parse_prim_int_type(
    parser: &mut ParseContext,
    ident: String,
    loc: Span,
) -> ParseResult<Located<Type>> {
    let mut it = ident.chars();
    let _int_kind = it.next().unwrap();
    let bit_width = it.collect::<String>().parse::<u16>().unwrap();
    if bit_width > 64 {
        parser.push_error(Located(
            ParseError::TypeParseError(TypeParseError::IntegerSizeOutOfRange(bit_width)),
            loc,
        ));
        return None;
    }

    todo!("n-sized integer type parse")
    //Some(Located(
    //    parser.type_context.get_primitive(match int_kind {
    //        'i' => PrimType::SignedInt(bit_width),
    //        'u' => PrimType::UnsignedInt(bit_width),
    //        _ => unimplemented!("integer kind {}", int_kind),
    //    }),
    //    loc,
    //))
}

fn parse_ident_type(parser: &mut ParseContext) -> ParseResult<Located<Type>> {
    let Token {
        kind: TokenKind::Identifier(Identifier(ident)),
        loc,
    } = parser.lexer.next_token()
    else {
        panic!("token should be a type keyword");
    };

    if ['u', 'i'].contains(&ident.chars().next().unwrap_or('\0'))
        && ident.chars().skip(1).all(char::is_numeric)
    {
        return parse_prim_int_type(parser, ident, loc);
    }
    todo!("ident type parse")
}

static PARSER: Lazy<PrattParser<Located<Type>, BindingPower>> =
    Lazy::new(|| PrattParser::<Located<Type>, BindingPower>::new(&TypeHandlers));

impl Parse for Located<Type> {
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
impl TryParse for Located<Type> {
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
