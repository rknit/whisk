use std::io::Read;

use super::{errors::ParseErrorReport, parser::Parser};

pub mod func;

pub trait Parse {
    type Output;
    fn parse<R: Read>(parser: &mut Parser<R>) -> Result<Self::Output, ParseErrorReport>;
}
