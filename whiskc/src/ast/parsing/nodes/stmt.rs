use once_cell::sync::Lazy;

use crate::{
    ast::{
        location::{Located, LocationRange},
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
        handler(TokenKind::Delimiter(Delimiter::BraceOpen), |_, parser| {
            Block::parse(parser).map(|v| Stmt::Block(v))
        });

        handler(TokenKind::Keyword(Keyword::Let), |_, parser| {
            LetStmt::parse(parser).map(|v| Stmt::Let(v))
        });

        handler(TokenKind::Keyword(Keyword::If), |_, parser| {
            IfStmt::parse(parser).map(|v| Stmt::If(v))
        });

        handler(TokenKind::Keyword(Keyword::Return), |_, parser| {
            ReturnStmt::parse(parser).map(|v| Stmt::Return(v))
        });

        handler(TokenKind::Keyword(Keyword::Loop), |_, parser| {
            LoopStmt::parse(parser).map(|v| Stmt::Loop(v))
        });
    }
}

impl Parse for Block {
    fn parse(parser: &mut ParseContext) -> ParseResult<Block> {
        let brace_open_tok = match_delimiter!(parser, Delimiter::BraceOpen =>);

        let mut stmts = Vec::new();
        let brace_close_tok = loop {
            if let Ok(b) = match_delimiter!(parser, Delimiter::BraceClose) {
                break b;
            }
            if match_token_kind!(parser, TokenKind::EndOfFile).is_some() {
                break match_delimiter!(parser, Delimiter::BraceClose =>);
            }

            let stmt = Stmt::parse(parser);
            if let Some(stmt) = stmt {
                stmts.push(stmt);
                continue;
            }

            // panic mode: skip until next semicolon or brace_close or EOF
            while !matches!(
                parser.lexer.peek_token_kind(0),
                TokenKind::EndOfFile
                    | TokenKind::Delimiter(Delimiter::Semicolon | Delimiter::BraceClose)
            ) {
                parser.lexer.next_token();
            }

            // panic mode: found semicolon, proceed to next stmt
            if matches!(
                parser.lexer.peek_token_kind(0),
                TokenKind::Delimiter(Delimiter::Semicolon)
            ) {
                parser.lexer.next_token();
            }
        };

        Some(Self {
            brace_open_tok: brace_open_tok.into(),
            stmts,
            brace_close_tok: brace_close_tok.into(),
        })
    }
}

fn parse_expr_stmt_or_assign_stmt(parser: &mut ParseContext) -> ParseResult<Stmt> {
    let expr = Expr::parse(parser)?;
    Some(
        if let Ok(assign_tok) = match_operator!(parser, Operator::Assign) {
            let value = Expr::parse(parser)?;
            let semi_tok = match_delimiter!(parser, Delimiter::Semicolon =>);
            Stmt::Assign(AssignStmt {
                target: expr,
                assign_tok: assign_tok.into(),
                value,
                semi_tok: semi_tok.into(),
            })
        } else {
            let semi_tok = match_delimiter!(parser, Delimiter::Semicolon =>);
            Stmt::Expr(ExprStmt {
                expr,
                semi_tok: semi_tok.into(),
            })
        },
    )
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

impl Parse for IfStmt {
    fn parse(parser: &mut ParseContext) -> ParseResult<Self> {
        let if_tok = match_keyword!(parser, Keyword::If =>);
        let cond = Expr::parse(parser)?;
        let block = Block::parse(parser)?;
        let else_stmt = if matches!(parser.lexer.peek_token_kind(0), TokenKind::Keyword(kw) if *kw == Keyword::Else)
        {
            ElseStmt::parse(parser)
        } else {
            None
        };
        Some(Self {
            if_tok: if_tok.into(),
            cond,
            body: block,
            else_stmt,
        })
    }
}

impl Parse for ElseStmt {
    fn parse(ctx: &mut ParseContext) -> ParseResult<Self> {
        let else_tok = match_keyword!(ctx, Keyword::Else =>);
        let block = Block::parse(ctx)?;
        Some(Self {
            else_tok,
            body: block,
        })
    }
}

impl Parse for ReturnStmt {
    fn parse(parser: &mut ParseContext) -> ParseResult<Self> {
        let return_tok = match_keyword!(parser, Keyword::Return =>);
        Some(
            if let Ok(semi_tok) = match_delimiter!(parser, Delimiter::Semicolon) {
                Self {
                    return_tok: return_tok.into(),
                    expr: None,
                    semi_tok: semi_tok.into(),
                }
            } else {
                let expr = Expr::parse(parser)?;
                let semi_tok = match_delimiter!(parser, Delimiter::Semicolon =>);
                Self {
                    return_tok: return_tok.into(),
                    expr: Some(expr),
                    semi_tok: semi_tok.into(),
                }
            },
        )
    }
}

impl Parse for LoopStmt {
    fn parse(ctx: &mut ParseContext) -> ParseResult<Self> {
        let loop_tok = match_keyword!(ctx, Keyword::Loop =>);
        let block = Block::parse(ctx)?;
        Some(Self { loop_tok, block })
    }
}

impl Parse for Stmt {
    fn parse(parser: &mut ParseContext) -> ParseResult<Stmt> {
        static PARSER: Lazy<LookUpParser<Stmt>> =
            Lazy::new(|| LookUpParser::<Stmt>::new(&StmtHandlers));

        let prev_loc: LocationRange = *parser.lexer.get_prev_loc();

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
            parse_expr_stmt_or_assign_stmt(parser)
        }
    }
}
