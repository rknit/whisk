use crate::ast::{
    nodes::{
        attributes::Attributes,
        ty::{Type, TypeDecl, TypeDeclKind},
    },
    parsing::{
        token::{Delimiter, Keyword, Operator, TokenKind},
        Parse, ParseContext, ParseResult,
    },
};

use super::ty::parse_struct_type;

impl Parse for TypeDecl {
    fn parse(ctx: &mut ParseContext) -> ParseResult<Self> {
        let ty_tok = match_keyword!(ctx, Keyword::Type =>);
        let name = match_identifier!(ctx, "type's name".to_owned() =>)?;
        let assign_tok = match_operator!(ctx, Operator::Assign =>);

        let kind: TypeDeclKind = if matches!(
            ctx.lexer.peek_token_kind(0),
            TokenKind::Keyword(Keyword::Struct)
        ) {
            parse_struct_type(ctx)?.into()
        } else {
            Type::parse(ctx)?.into()
        };

        let semi_tok = match_delimiter!(ctx, Delimiter::Semicolon =>);
        Some(TypeDecl {
            attributes: Attributes::default(),
            ty_tok,
            name,
            assign_tok,
            kind,
            semi_tok,
        })
    }
}
