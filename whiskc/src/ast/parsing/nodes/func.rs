use crate::ast::{
    location::Located,
    nodes::{
        attributes::Attributes,
        expr::Expr,
        func::{ExternFunction, Function, FunctionSig, Param},
        punctuate::Punctuated,
        ty::{PrimType, Type},
    },
    parsing::{
        token::{Delimiter, Keyword, TokenKind},
        Parse, ParseContext, ParseError, ParseResult,
    },
};

impl Parse for Function {
    fn parse(ctx: &mut ParseContext) -> ParseResult<Self> {
        let sig = FunctionSig::parse(ctx)?;
        let Expr::Block(body) = Expr::parse(ctx)? else {
            ctx.push_error(Located(
                ParseError::FuncParseError(FunctionParseError::MissingFunctionBody {
                    func_name: sig.name.0.clone(),
                }),
                sig.name.1,
            ));
            return None;
        };
        Some(Self { sig, body })
    }
}

impl Parse for ExternFunction {
    fn parse(ctx: &mut ParseContext) -> ParseResult<Self> {
        let extern_tok = match_keyword!(ctx, Keyword::Extern =>);
        let sig = FunctionSig::parse(ctx)?;
        let semi_tok = match_delimiter!(ctx, Delimiter::Semicolon =>);
        Some(Self {
            extern_tok,
            sig,
            semi_tok,
        })
    }
}

impl Parse for FunctionSig {
    fn parse(ctx: &mut ParseContext) -> ParseResult<Self> {
        let func_tok = match_keyword!(ctx, Keyword::Func =>);

        let name = match_identifier!(ctx, "function name".to_owned() =>)?;

        let paren_open_tok = match_delimiter!(ctx, Delimiter::ParenOpen =>);
        let params = Punctuated::parse(ctx, Delimiter::Comma, Delimiter::ParenClose, move |ctx| {
            let param_name = match_identifier!(ctx, "parameter name".to_owned() =>)?;
            let param_ty = Type::parse(ctx)?;
            Some(Param(param_name, param_ty))
        })?;
        let paren_close_tok = match_delimiter!(ctx, Delimiter::ParenClose =>);

        let ret_ty = if matches!(
            ctx.lexer.peek_token_kind(0),
            TokenKind::Delimiter(Delimiter::BraceOpen)
        ) {
            Type::Primitive(Located::new_temp(PrimType::Unit))
        } else {
            Type::parse(ctx)?
        };

        Some(Self {
            attributes: Attributes::default(),
            func_tok,
            name,
            paren_open_tok,
            params,
            paren_close_tok,
            ret_ty,
        })
    }
}

#[derive(Debug, Clone)]
pub enum FunctionParseError {
    MissingFunctionBody { func_name: String },
}
