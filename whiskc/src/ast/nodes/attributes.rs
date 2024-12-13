use core::fmt;

use crate::ast::{location::Located, parsing::token::Keyword};

#[derive(Clone, Default)]
pub struct Attributes {
    pub attribs: Vec<Located<Keyword>>,
}
impl fmt::Debug for Attributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.attribs)
    }
}
