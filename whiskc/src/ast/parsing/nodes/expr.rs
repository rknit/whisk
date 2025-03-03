use once_cell::sync::Lazy;

use crate::ast::{
    location::{Locatable, Located, Span},
    nodes::{
        expr::*,
        punctuate::Punctuated,
        stmt::{ExprStmt, Stmt},
    },
    parsing::{
        parsers::pratt_parser::{self, PrattParseError, PrattParseResult, PrattParser},
        token::{
            Delimiter, Identifier, Keyword, Literal, LiteralKeyword, Operator, Token, TokenKind,
        },
        Parse, ParseContext, ParseError, ParseResult,
    },
};

#[derive(Debug, Clone)]
pub enum ExprParseError {
    UnexpectedToken(TokenKind),
    UnexpectedInfixOperator(TokenKind),
    ExpectedIdentForStructInit(Expr),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum BindingPower {
    Zero = 0,
    Assign,
    LogicalAdditive,
    LogicalMultiplicative,
    Comparative,
    Modulus,
    Additive,
    Multiplicative,
    Unary,
    Call,
    MemberAccess,
    StructInit,
    Primary,
}

struct ExprHandlers;
impl pratt_parser::Handlers<Expr, BindingPower> for ExprHandlers {
    fn nuds<F>(&self, mut nud: F)
    where
        F: FnMut(TokenKind, pratt_parser::NudHandler<Expr, BindingPower>),
    {
        let primaries = [
            TokenKind::LiteralKeyword(LiteralKeyword::True),
            TokenKind::LiteralKeyword(LiteralKeyword::False),
            TokenKind::Literal(Literal::Int(0)),
            TokenKind::Identifier(Identifier("".into())),
        ];
        for primary in primaries {
            nud(primary, parse_primary_expr);
        }

        let prefix_unary_ops = [Operator::Sub, Operator::Not];
        for op in prefix_unary_ops {
            nud(TokenKind::Operator(op), parse_prefix_unary_expr);
        }

        nud(
            TokenKind::Delimiter(Delimiter::ParenOpen),
            parse_unit_or_group_expr,
        );

        nud(TokenKind::Delimiter(Delimiter::BraceOpen), parse_block_expr);

        nud(TokenKind::Keyword(Keyword::Return), parse_return_expr);
        nud(TokenKind::Keyword(Keyword::If), parse_if_expr);
        nud(TokenKind::Keyword(Keyword::Loop), parse_loop_expr);
    }

    fn leds<F>(&self, mut led: F)
    where
        F: FnMut(TokenKind, BindingPower, pratt_parser::LedHandler<Expr, BindingPower>),
    {
        let bin_ops = [
            (BindingPower::Additive, vec![Operator::Add, Operator::Sub]),
            (
                BindingPower::Multiplicative,
                vec![Operator::Mul, Operator::Div],
            ),
            (BindingPower::Modulus, vec![Operator::Mod]),
            (BindingPower::LogicalMultiplicative, vec![Operator::And]),
            (BindingPower::LogicalAdditive, vec![Operator::Or]),
            (
                BindingPower::Comparative,
                vec![
                    Operator::Equal,
                    Operator::NotEqual,
                    Operator::Less,
                    Operator::LessEqual,
                    Operator::Greater,
                    Operator::GreaterEqual,
                ],
            ),
            (BindingPower::Assign, vec![Operator::Assign]),
        ];
        for (bp, ops) in bin_ops {
            for op in ops {
                led(TokenKind::Operator(op), bp, parse_binary_expr);
            }
        }

        led(
            TokenKind::Delimiter(Delimiter::ParenOpen),
            BindingPower::Call,
            parse_call_expr,
        );

        led(
            TokenKind::Delimiter(Delimiter::BraceOpen),
            BindingPower::StructInit,
            parse_struct_init_expr,
        );

        led(
            TokenKind::Operator(Operator::MemberAccess),
            BindingPower::MemberAccess,
            parse_member_access_expr,
        );
    }
}

fn parse_unit_or_group_expr(
    _pratt_parser: &PrattParser<Expr, BindingPower>,
    parser: &mut ParseContext,
) -> ParseResult<Expr> {
    let paren_open_tok = match_delimiter!(parser, Delimiter::ParenOpen =>);
    Some(
        if let Ok(paren_close_tok) = match_delimiter!(parser, Delimiter::ParenClose) {
            Expr::Unit(Span::combine(paren_open_tok.1, paren_close_tok.1))
        } else {
            let expr = Expr::parse(parser)?;
            let paren_close_tok = match_delimiter!(parser, Delimiter::ParenClose =>);
            Expr::Grouped(GroupedExpr {
                paren_open_tok,
                expr: Box::new(expr),
                paren_close_tok,
            })
        },
    )
}

fn parse_primary_expr(
    _pratt_parser: &PrattParser<Expr, BindingPower>,
    parser: &mut ParseContext,
) -> ParseResult<Expr> {
    let tok = parser.lexer.next_token();

    Some(match tok.kind {
        TokenKind::Literal(lit) => match lit {
            Literal::Int(v) => Expr::Integer(Located(v, tok.loc)),
        },
        TokenKind::LiteralKeyword(kw) => match kw {
            LiteralKeyword::True => Expr::Bool(Located(true, tok.loc)),
            LiteralKeyword::False => Expr::Bool(Located(false, tok.loc)),
        },
        TokenKind::Identifier(Identifier(ident)) => Expr::Ident(Located(ident, tok.loc)),
        _ => unimplemented!("{:#?}", tok),
    })
}

fn parse_prefix_unary_expr(
    pratt_parser: &PrattParser<Expr, BindingPower>,
    parser: &mut ParseContext,
) -> ParseResult<Expr> {
    let Token {
        kind: TokenKind::Operator(op),
        loc,
    } = parser.lexer.next_token()
    else {
        panic!("non operator token is not supported");
    };

    let expr = Expr::handle_err(pratt_parser.parse(parser, BindingPower::Unary), parser)?;
    Some(Expr::Unary(UnaryExpr {
        op: Located(op, loc),
        expr: Box::new(expr),
    }))
}

fn parse_binary_expr(
    pratt_parser: &PrattParser<Expr, BindingPower>,
    parser: &mut ParseContext,
    left: Expr,
    bp: BindingPower,
) -> ParseResult<Expr> {
    let Token {
        kind: TokenKind::Operator(op),
        loc,
    } = parser.lexer.next_token()
    else {
        panic!("non operator token is not supported");
    };

    let right = Expr::handle_err(pratt_parser.parse(parser, bp), parser)?;
    Some(Expr::Binary(BinaryExpr {
        op: Located(op, loc),
        left: Box::new(left),
        right: Box::new(right),
    }))
}

fn parse_call_expr(
    _pratt_parser: &PrattParser<Expr, BindingPower>,
    parser: &mut ParseContext,
    left: Expr,
    _bp: BindingPower,
) -> ParseResult<Expr> {
    let paren_open_tok = match_delimiter!(parser, Delimiter::ParenOpen =>);
    let args = Punctuated::parse(parser, Delimiter::Comma, Delimiter::ParenClose, Expr::parse)?;
    let paren_close_tok = match_delimiter!(parser, Delimiter::ParenClose =>);
    Some(Expr::Call(CallExpr {
        caller: Box::new(left),
        paren_open_tok,
        args,
        paren_close_tok,
    }))
}

fn parse_block_expr(
    _pratt_parser: &PrattParser<Expr, BindingPower>,
    parser: &mut ParseContext,
) -> ParseResult<Expr> {
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

    let eval_expr = if matches!(
        stmts.last(),
        Some(Stmt::Expr(ExprStmt { semi_tok: None, expr })) if expr.has_eval_expr() || (!expr.has_eval_expr() && matches!(expr, Expr::Loop(_))) || !expr.is_block()
    ) {
        let Some(Stmt::Expr(ExprStmt {
            expr,
            semi_tok: None,
        })) = stmts.pop()
        else {
            unreachable!()
        };
        Some(Box::new(expr))
    } else {
        None
    };

    for stmt in &stmts {
        let Stmt::Expr(ExprStmt { expr, semi_tok }) = stmt else {
            continue;
        };
        if !expr.is_block() && semi_tok.is_none() {
            parser.push_error(Located(
                ParseError::MissingDelimiter(Delimiter::Semicolon),
                expr.get_location().next().into(),
            ));
        }
    }

    Some(Expr::Block(BlockExpr {
        brace_open_tok,
        stmts,
        eval_expr,
        brace_close_tok,
    }))
}

fn parse_return_expr(
    _pratt_parser: &PrattParser<Expr, BindingPower>,
    parser: &mut ParseContext,
) -> ParseResult<Expr> {
    let return_tok = match_keyword!(parser, Keyword::Return =>);
    let expr = if matches!(
        parser.lexer.peek_token_kind(0),
        TokenKind::Delimiter(
            Delimiter::Semicolon
                | Delimiter::ParenClose
                | Delimiter::BraceClose
                | Delimiter::BracketClose
                | Delimiter::Comma
        )
    ) {
        None
    } else {
        Some(Box::new(Expr::parse(parser)?))
    };

    Some(Expr::Return(ReturnExpr { return_tok, expr }))
}

fn parse_if_expr(
    pratt_parser: &PrattParser<Expr, BindingPower>,
    parser: &mut ParseContext,
) -> ParseResult<Expr> {
    let if_tok = match_keyword!(parser, Keyword::If =>);
    let cond = Expr::parse(parser)?;
    let Expr::Block(block) = parse_block_expr(pratt_parser, parser)? else {
        unreachable!();
    };
    let else_expr = if matches!(parser.lexer.peek_token_kind(0), TokenKind::Keyword(kw) if *kw == Keyword::Else)
    {
        parse_else_expr(pratt_parser, parser)
    } else {
        None
    };
    Some(Expr::If(IfExpr {
        if_tok,
        cond: Box::new(cond),
        then: block,
        else_expr,
    }))
}

fn parse_else_expr(
    pratt_parser: &PrattParser<Expr, BindingPower>,
    parser: &mut ParseContext,
) -> ParseResult<ElseExpr> {
    let else_tok = match_keyword!(parser, Keyword::Else =>);
    let Expr::Block(block) = parse_block_expr(pratt_parser, parser)? else {
        unreachable!()
    };
    Some(ElseExpr {
        else_tok,
        body: block,
    })
}

fn parse_loop_expr(
    pratt_parser: &PrattParser<Expr, BindingPower>,
    parser: &mut ParseContext,
) -> ParseResult<Expr> {
    let loop_tok = match_keyword!(parser, Keyword::Loop =>);
    let Expr::Block(block) = parse_block_expr(pratt_parser, parser)? else {
        unreachable!()
    };
    Some(Expr::Loop(LoopExpr {
        loop_tok,
        body: block,
    }))
}

fn parse_struct_init_expr(
    _pratt_parser: &PrattParser<Expr, BindingPower>,
    parser: &mut ParseContext,
    left: Expr,
    _bp: BindingPower,
) -> ParseResult<Expr> {
    let Expr::Ident(ident) = left else {
        let loc = left.get_location();
        parser.push_error(Located(
            ParseError::ExprParseError(ExprParseError::ExpectedIdentForStructInit(left)),
            loc,
        ));
        return None;
    };

    let brace_open_tok = match_delimiter!(parser, Delimiter::BraceOpen =>);
    let fields = Punctuated::parse(
        parser,
        Delimiter::Comma,
        Delimiter::BraceClose,
        FieldInit::parse,
    )?;
    let brace_close_tok = match_delimiter!(parser, Delimiter::BraceClose =>);

    Some(Expr::StructInit(StructInitExpr {
        ty_name: ident,
        brace_open_tok,
        fields,
        brace_close_tok,
    }))
}

impl Parse for FieldInit {
    fn parse(ctx: &mut ParseContext) -> ParseResult<Self> {
        let field_name = match_identifier!(ctx, "field name".to_owned() =>)?;
        let colon_tok = match_delimiter!(ctx, Delimiter::Colon =>);
        let expr = Expr::parse(ctx)?;
        Some(Self {
            field_name,
            colon_tok,
            expr,
        })
    }
}

fn parse_member_access_expr(
    _pratt_parser: &PrattParser<Expr, BindingPower>,
    parser: &mut ParseContext,
    left: Expr,
    _bp: BindingPower,
) -> ParseResult<Expr> {
    let mem_access_op = match_operator!(parser, Operator::MemberAccess =>);
    let field_name = match_identifier!(parser, "field name or method name".to_owned() =>)?;

    Some(Expr::MemberAccess(MemberAccessExpr {
        expr: Box::new(left),
        mem_access_op,
        field_name,
    }))
}

impl Expr {
    pub fn parse(parser: &mut ParseContext) -> ParseResult<Expr> {
        static EXPR_PARSER: Lazy<PrattParser<Expr, BindingPower>> =
            Lazy::new(|| PrattParser::new(&ExprHandlers));
        Self::handle_err(EXPR_PARSER.parse(parser, BindingPower::Zero), parser)
    }

    fn handle_err(e: PrattParseResult<Expr>, parser: &mut ParseContext) -> ParseResult<Expr> {
        let Err(e) = e else {
            return e.ok();
        };
        match e {
            PrattParseError::NoNudHandlerFound(e) => parser.push_error(Located(
                ParseError::ExprParseError(ExprParseError::UnexpectedToken(e.kind)),
                e.loc,
            )),
            PrattParseError::NoLedHandlerFound(e) => parser.push_error(Located(
                ParseError::ExprParseError(ExprParseError::UnexpectedInfixOperator(e.kind)),
                e.loc,
            )),
            PrattParseError::ParseError => (),
        }
        None
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
