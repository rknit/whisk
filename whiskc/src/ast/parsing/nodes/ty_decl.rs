use crate::ast::{
    nodes::{
        attributes::Attributes,
        ty::{Type, TypeDecl},
    },
    parsing::{
        token::{Delimiter, Keyword, Operator},
        Parse, ParseContext, ParseResult,
    },
};

impl Parse for TypeDecl {
    fn parse(ctx: &mut ParseContext) -> ParseResult<Self> {
        let ty_tok = match_keyword!(ctx, Keyword::Type =>);
        let name = match_identifier!(ctx, "type's name".to_owned() =>)?;
        let assign_tok = match_operator!(ctx, Operator::Assign =>);
        let ty = Type::parse(ctx)?;
        let semi_tok = match_delimiter!(ctx, Delimiter::Semicolon =>);
        Some(TypeDecl {
            attributes: Attributes::default(),
            ty_tok,
            name,
            assign_tok,
            ty,
            semi_tok,
        })
    }
}
