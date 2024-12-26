use std::io::Read;

use super::{errors::ParseError, lexer::Lexer};

#[derive(Debug)]
pub struct Parser<R: Read>(pub Lexer<R>, pub ParseContext);

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

#[macro_export]
macro_rules! match_token_kind {
    ($parser:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {{
        if matches!($parser.0.peek_token_kind(), $pattern $(if $guard)?) {
            Some($parser.0.next_token())
        } else {
            None
        }
    }};
}
