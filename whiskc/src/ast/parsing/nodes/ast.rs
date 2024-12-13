use std::path::Path;

use crate::ast::{
    location::Located,
    nodes::item::Item,
    parsing::{lexer::Lexer, Parse, ParseContext, ParseError},
    AST,
};

pub fn parse(source: &Path) -> Result<AST, Vec<Located<ParseError>>> {
    let mut ctx = ParseContext::new(Lexer::new(source));

    let mut items = Vec::new();
    while !ctx.lexer.is_eof() {
        let item = Item::parse(&mut ctx);
        if let Some(item) = item {
            items.push(item);
        } else {
            ctx.lexer.next_token();
        }
    }

    ctx.finalize()?;
    Ok(AST { items })
}
