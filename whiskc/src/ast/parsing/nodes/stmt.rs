use once_cell::sync::Lazy;

use crate::{
    ast::{
        location::{Located, Span},
        nodes::{expr::Expr, stmt::*},
        parsing::{
            parsers::lookup_parser::{self, LookUpParseError, LookUpParser},
            token::{Delimiter, Keyword, Operator, TokenKind},
            Parse, ParseContext, ParseError, ParseResult, TryParse,
        },
    },
    ty::Type,
};

#[derive(Debug, Clone)]
pub enum StmtParseError {
    UnexpectedToken(TokenKind),
}

struct StmtHandlers;
impl lookup_parser::Handlers<Stmt> for StmtHandlers {
    fn handlers<F>(&self, mut handler: F)
    where
        F: FnMut(TokenKind, lookup_parser::Handler<Stmt>),
    {
        handler(TokenKind::Keyword(Keyword::Let), |_, parser| {
            LetStmt::parse(parser).map(Stmt::Let)
        });
    }
}

fn parse_expr_stmt(parser: &mut ParseContext) -> ParseResult<Stmt> {
    let expr = Expr::parse(parser)?;
    Some(Stmt::Expr(ExprStmt {
        expr,
        semi_tok: match_delimiter!(parser, Delimiter::Semicolon).ok(),
    }))
}

impl Parse for LetStmt {
    fn parse(parser: &mut ParseContext) -> ParseResult<Self> {
        let let_tok = match_keyword!(parser, Keyword::Let =>);
        let name = match_identifier!(parser, "let declaration's name".to_owned() =>)?;
        let ty = Located::<Type>::try_parse(parser);
        let assign_tok = match_operator!(parser, Operator::Assign =>);
        let value = Expr::parse(parser)?;
        let semi_tok = match_delimiter!(parser, Delimiter::Semicolon =>);

        Some(Self {
            let_tok,
            name,
            ty,
            assign_tok,
            value,
            semi_tok,
        })
    }
}

impl Parse for Stmt {
    fn parse(parser: &mut ParseContext) -> ParseResult<Stmt> {
        static PARSER: Lazy<LookUpParser<Stmt>> =
            Lazy::new(|| LookUpParser::<Stmt>::new(&StmtHandlers));

        let prev_loc: Span = *parser.lexer.get_prev_loc();

        let parsed_lookup_stmt = PARSER.parse(parser);

        if *parser.lexer.get_prev_loc() != prev_loc {
            match parsed_lookup_stmt {
                Ok(v) => Some(v),
                Err(e) => {
                    match e {
                        LookUpParseError::NoHandlerFound(t) => parser.push_error(Located(
                            ParseError::StmtParseError(StmtParseError::UnexpectedToken(t.kind)),
                            t.loc,
                        )),
                        LookUpParseError::ParseError => (),
                    };
                    None
                }
            }
        } else {
            parse_expr_stmt(parser)
        }
    }
}
