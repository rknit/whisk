use crate::ast::location::Located;

use super::token::{Delimiter, Keyword};

#[derive(Debug, Default)]
pub struct ParseErrorReport {
    errs: Vec<ParseError>,
}
impl ParseErrorReport {
    pub fn is_ok(&self) -> bool {
        self.errs.is_empty()
    }

    pub fn push(&mut self, e: impl Into<ParseError>) {
        self.errs.push(e.into());
    }

    pub fn merge(&mut self, mut other: Self) {
        self.errs.append(&mut other.errs)
    }

    pub fn finalize(self) -> Vec<ParseError> {
        self.errs
    }
}

#[derive(Debug)]
pub enum ParseError {
    MissingDelim(Located<Delimiter>),
    MissingIdent(Located<&'static str>),
    MissingKeyword(Located<Keyword>),
}
