use std::io::Read;

use self::{errors::ParseError, lexer::Lexer};

pub mod errors;
pub mod lexer;
pub mod token;

#[derive(Debug, Default)]
pub struct ParseContext {
    errors: Vec<ParseError>,
}
impl ParseContext {
    pub fn push_error(&mut self, e: impl Into<ParseError>) {
        self.errors.push(e.into());
    }

    pub fn finalize(self) -> Result<(), Vec<ParseError>> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors)
        }
    }
}

#[derive(Debug)]
pub struct Parser<R: Read>(pub Lexer<R>, pub ParseContext);

/// Parse trait does not guarantee whether the tokens will be consumed or not when a parse failure
/// occurs.
/// This trait will also append errors to error list in case of parse failures.
pub trait Parse {
    type Output;
    fn parse<R: Read>(parser: &mut Parser<R>) -> Option<Self::Output>;
}

/// TryParse trait will perserve the tokens if it cannot parse successfully.
/// This trait will *not* append errors to error list in case of parse failures.
pub trait TryParse {
    type Output;
    fn try_parse<R: Read>(parser: &mut Parser<R>) -> Option<Self::Output>;
}
