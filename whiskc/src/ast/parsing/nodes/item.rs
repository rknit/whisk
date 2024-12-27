use crate::ast::{
    location::Located,
    nodes::{
        attributes::Attributes,
        func::{ExternFunction, Function},
        item::Item,
    },
    parsing::{
        token::{Keyword, TokenKind},
        Parse, ParseError,
    },
};

impl Parse for Item {
    fn parse(
        ctx: &mut crate::ast::parsing::ParseContext,
    ) -> crate::ast::parsing::ParseResult<Self> {
        let attributes = Attributes::parse(ctx).unwrap_or_default();

        let item = match ctx.lexer.peek_token_kind(0) {
            TokenKind::Keyword(kw) => match kw {
                Keyword::Extern => {
                    let mut func = ExternFunction::parse(ctx)?;
                    func.sig.attributes = attributes;
                    Some(Item::ExternFunction(func))
                }
                Keyword::Func => {
                    let mut func = Function::parse(ctx)?;
                    func.sig.attributes = attributes;
                    Some(Item::Function(func))
                }
                _ => None,
            },
            _ => None,
        };
        if item.is_some() {
            return item;
        }

        let tok = ctx.lexer.peek_token(0).clone();
        ctx.push_error(Located(
            ParseError::ItemParseError(ItemParseError::UnexpectedToken(tok.kind)),
            tok.loc,
        ));
        None
    }
}

#[derive(Debug, Clone)]
pub enum ItemParseError {
    UnexpectedToken(TokenKind),
}
