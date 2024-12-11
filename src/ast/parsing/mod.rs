use crate::ast::parsing::nodes::ty::TypeParseError;

use super::{
    location::Located,
    parsing::{
        lexer::Lexer,
        nodes::{expr::ExprParseError, item::ItemParseError, stmt::StmtParseError},
        token::{Delimiter, Keyword, Operator},
    },
};

#[macro_use]
pub mod lexer;
pub mod nodes;
pub mod parsers;
pub mod token;

#[derive(Debug)]
pub struct ParseContext {
    errors: Vec<Located<ParseError>>,
    pub lexer: Lexer,
}
impl ParseContext {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            errors: Vec::new(),
            lexer,
        }
    }

    //pub fn new_block(&mut self) -> BlockID {
    //    let id = self.block_id_counter;
    //    let parent_id = *self.block_id_stack.last().unwrap();
    //    self.sym_table.new_local_scope(id, parent_id);
    //    self.block_id_counter += 1;
    //    id
    //}
    //
    //pub fn push_block_id(&mut self, id: BlockID) {
    //    self.block_id_stack.push(id);
    //}
    //
    //pub fn pop_block_id(&mut self) {
    //    debug_assert!(
    //        self.block_id_stack.len() >= 1,
    //        "popping global scope is not allowed"
    //    );
    //    self.block_id_stack.pop();
    //}

    pub fn push_error(&mut self, e: Located<ParseError>) {
        self.errors.push(e);
    }

    pub fn finalize(self) -> Result<(), Vec<Located<ParseError>>> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors)
        }
    }
}

#[derive(Debug, Clone)]
pub enum ParseError {
    MissingKeyword(Keyword),
    MissingDelimiter(Delimiter),
    MissingIdentifier(String),
    MissingOperator(Operator),
    ItemParseError(ItemParseError),
    TypeParseError(TypeParseError),
    StmtParseError(StmtParseError),
    ExprParseError(ExprParseError),
}

pub type ParseResult<T> = Option<T>;

pub type TryParseResult<T> = Option<T>;

pub trait Parse {
    /// parse() will append errors into the error log when it fails to match the parse rules
    fn parse(ctx: &mut ParseContext) -> ParseResult<Self>
    where
        Self: Sized;
}

pub trait TryParse {
    /// try_parse() will *not* append errors into the error log when it fails to match the parse rules
    fn try_parse(ctx: &mut ParseContext) -> TryParseResult<Self>
    where
        Self: Sized;
}
