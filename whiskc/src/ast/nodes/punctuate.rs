use crate::ast::parsing::token::Delimiter;

#[derive(Debug, Clone)]
pub struct Punctuated<T> {
    pub items: Vec<T>,
    pub sep: Delimiter,
}
