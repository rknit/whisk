use crate::{
    ast::{
        location::Located,
        nodes::{
            attributes::Attributes,
            func::{ExternFunction, Function, FunctionSig, LocatedParam},
            punctuate::Puntuated,
            stmt::Block,
        },
        parsing::{
            token::{Delimiter, Keyword, TokenKind},
            Parse, ParseContext, ParseResult,
        },
    },
    ty::{PrimType, Type},
};

impl Parse for Function {
    fn parse(ctx: &mut ParseContext) -> ParseResult<Self> {
        let sig = FunctionSig::parse(ctx)?;
        let body = Block::parse(ctx)?;
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
        let params = Puntuated::parse(ctx, Delimiter::Comma, Delimiter::ParenClose, move |ctx| {
            let param_name = match_identifier!(ctx, "parameter name".to_owned() =>)?;
            let param_ty = Located::<Type>::parse(ctx)?;
            Some(LocatedParam(param_name.into(), param_ty))
        })?;
        let paren_close_tok = match_delimiter!(ctx, Delimiter::ParenClose =>);

        let ret_ty = if matches!(
            ctx.lexer.peek_token_kind(0),
            TokenKind::Delimiter(Delimiter::BraceOpen)
        ) {
            Located::new_temporary(PrimType::Unit.into())
        } else {
            Located::<Type>::parse(ctx)?
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
