use std::io::Read;

use super::{
    errors::{ParseError, ParseErrorReport},
    lexer::Lexer,
};

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

pub trait ReportErr {
    type Output;
    fn report(self, rep: &mut ParseErrorReport) -> Option<Self::Output>;
}
impl<T, U: Into<ParseError>> ReportErr for Result<T, U> {
    type Output = T;

    fn report(self, rep: &mut ParseErrorReport) -> Option<Self::Output> {
        match self {
            Ok(v) => Some(v),
            Err(e) => {
                rep.push(e);
                None
            }
        }
    }
}

#[macro_export]
macro_rules! match_token_kind {
    ($parser:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {{
        match $parser.0.peek_kind() {
            $pattern $(if $guard)? => Some($parser.0.next_token()),
            _ => None,
        }
    }};
}

#[macro_export]
macro_rules! match_identifier {
    ($parser:expr) => {{
        use $crate::ast::location::Located;
        use $crate::match_token_kind;
        use $crate::new_ast::token::{Token, TokenKind};
        if let Some(Token {
            kind: TokenKind::Identifier(ident),
            loc,
        }) = match_token_kind!($parser, TokenKind::Identifier(_))
        {
            Some(Located(ident, loc))
        } else {
            None
        }
    }};
    ($parser:expr => $expect:expr) => {{
        use $crate::ast::location::Located;
        use $crate::new_ast::errors::ParseError;
        match_identifier!($parser).ok_or_else(|| {
            ParseError::MissingIdent(Located($expect, $parser.0.get_last_loc().next().into()))
        })
    }};
    ($parser:expr => $expect:expr, $rep:expr) => {{
        use $crate::ast::location::Located;
        match match_identifier!($parser => $expect) {
            Ok(v) => v,
            Err(e) => {
                $rep.push(e);
                Located::new_temp(String::from($expect))
            }
        }
    }};
}

#[macro_export]
macro_rules! match_unit_token_kind {
    ($parser:expr, $unit_kind:ident, $val:expr) => {{
        use $crate::new_ast::token::{Token, TokenKind};
        use $crate::ast::location::Located;
        use $crate::match_token_kind;
        if let Some(Token {
            kind: TokenKind::$unit_kind(v),
            loc,
        }) = match_token_kind!($parser, TokenKind::$unit_kind(v) if *v == $val)
        {
            Some(Located(v, loc))
        } else {
            None
        }
    }};
}

#[macro_export]
macro_rules! match_delim {
    ($parser:expr, $v:expr) => {{
        use $crate::ast::location::Located;
        use $crate::match_unit_token_kind;
        use $crate::new_ast::errors::ParseError;
        match_unit_token_kind!($parser, Delimiter, $v.clone()).ok_or_else(|| {
            ParseError::MissingDelim(Located($v, $parser.0.get_last_loc().next().into()))
        })
    }};
    ($parser:expr, $v:expr => $default:expr, $rep:expr) => {{
        use $crate::ast::location::Located;
        match match_delim!($parser, $v) {
            Ok(v) => v,
            Err(e) => {
                $rep.push(e);
                Located::new_temp($default)
            }
        }
    }};
    ($parser:expr, $v:expr => , $rep:expr) => {{
        match_delim!($parser, $v.clone() => $v, $rep)
    }};
}

#[macro_export]
macro_rules! match_keyword {
    ($parser:expr, $kw:expr) => {{
        use $crate::ast::location::Located;
        use $crate::match_unit_token_kind;
        use $crate::new_ast::errors::ParseError;
        match_unit_token_kind!($parser, Keyword, $kw.clone()).ok_or_else(|| {
            ParseError::MissingKeyword(Located($kw, $parser.0.get_last_loc().next().into()))
        })
    }};
    ($parser:expr, $kw:expr => $default:expr, $rep:expr) => {{
        use $crate::ast::location::Located;
        match match_keyword!($parser, $kw) {
            Ok(v) => v,
            Err(e) => {
                $rep.push(e);
                Located::new_temp($default)
            }
        }
    }};
    ($parser:expr, $kw:expr => , $rep:expr) => {{
        match_keyword!($parser, $kw.clone() => $kw, $rep)
    }};
}

#[macro_export]
macro_rules! match_operator {
    ($parser:expr, $op:expr) => {{
        use $crate::ast::location::Located;
        match_unit_token_kind!($parser, Operator, $op.clone()).ok_or_else(|| {
            Located(
                $crate::ast::parsing::ParseError::MissingOperator($op),
                $parser.lexer.get_prev_loc().next().into(),
            )
        })
    }};
    ($parser:expr, $op:expr, $($ops:expr),+) => {{
        match_operator!($parser, $op)
            $(.or_else(|_| match_operator!($parser, $ops)))+
    }};
    ($parser:expr, $($ops:expr),* => $default:expr) => {{
        use $crate::ast::location::Located;
        match match_operator!($parser, $($ops),*) {
            Ok(v) => v,
            Err(e) => {
                $parser.push_error(e);
                Located::new_temporary($default)
            }
        }
    }};
    ($parser:expr, $op:expr =>) => {{
        match_operator!($parser, $op.clone() => $op)
    }};
}
