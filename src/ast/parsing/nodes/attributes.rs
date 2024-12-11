use crate::ast::{
    nodes::attributes::Attributes,
    parsing::{
        token::{Keyword, TokenKind},
        Parse, ParseContext, ParseResult,
    },
};

impl Parse for Attributes {
    fn parse(ctx: &mut ParseContext) -> ParseResult<Self> {
        const KEYWORDS: [Keyword; 1] = [Keyword::Pub];

        let peek_match = |ctx: &mut ParseContext, kw: Keyword| {
            if matches!(ctx.lexer.peek_token_kind(0), TokenKind::Keyword(k) if *k == kw) {
                Some(match_keyword!(ctx, kw =>))
            } else {
                None
            }
        };

        let mut attribs = Vec::new();
        loop {
            let mut has_matched = false;

            for kw in KEYWORDS {
                if let Some(attrib) = peek_match(ctx, kw) {
                    attribs.push(attrib);
                    has_matched = true;
                    break;
                }
            }

            if !has_matched {
                break;
            }
        }

        Some(Attributes { attribs })
    }
}
