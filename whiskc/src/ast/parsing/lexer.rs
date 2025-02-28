use std::{fs, path::Path, str::FromStr};

use crate::ast::location::{Location, Span};

use super::token::{
    Delimiter, Identifier, Keyword, Literal, LiteralKeyword, Operator, OperatorChar, Token,
    TokenKind, TypeKeyword,
};

#[derive(Debug, Default)]
pub struct Lexer {
    source: String,
    index: usize,
    tokens: Vec<Token>,
    current_loc: Location,
    prev_loc: Span,
}
impl Lexer {
    pub fn new(source_path: &Path) -> Self {
        let source = fs::read_to_string(source_path).expect("valid path to source file");
        Lexer {
            source,
            current_loc: Location { line: 1, col: 1 },
            ..Default::default()
        }
    }

    pub fn is_eof(&self) -> bool {
        self.is_at_buffer_end(0)
    }

    pub fn is_at_buffer_end(&self, ahead: usize) -> bool {
        self.index + ahead >= self.source.len()
    }

    pub fn peek_token_kind(&mut self, ahead: usize) -> &TokenKind {
        &self.peek_token(ahead).kind
    }

    pub fn peek_loc(&mut self, ahead: usize) -> &Span {
        &self.peek_token(ahead).loc
    }

    pub fn get_prev_loc(&self) -> &Span {
        &self.prev_loc
    }

    pub fn peek_token(&mut self, ahead: usize) -> &Token {
        while self.tokens.len() <= ahead {
            let token = self.make_token();
            self.tokens.push(token);
            self.skip_comment_and_whitespace();
        }
        &self.tokens[0]
    }

    pub fn next_token(&mut self) -> Token {
        let token = if self.tokens.is_empty() {
            let token = self.make_token();
            self.skip_comment_and_whitespace();
            token
        } else {
            self.tokens.remove(0)
        };
        self.prev_loc = token.loc;
        token
    }

    fn make_token(&mut self) -> Token {
        self.skip_comment_and_whitespace();
        let start: Location = self.current_loc;

        if self.is_eof() {
            Token {
                kind: TokenKind::EndOfFile,
                loc: start.into(),
            }
        } else if self.is_peek_char_f(0, char::is_numeric) {
            let value = self.get_str_while(char::is_numeric).unwrap();
            let value = value.parse::<i64>().unwrap();
            Token {
                kind: TokenKind::Literal(Literal::Int(value)),
                loc: Span {
                    start,
                    end: self.current_loc.front(),
                },
            }
        } else if self.is_peek_char_f(0, |c| {
            OperatorChar::from_str(c.to_string().as_str()).is_ok()
        }) {
            let mut peek_op_chars = self
                .peek_str_while(|c| OperatorChar::from_str(c.to_string().as_str()).is_ok())
                .unwrap();

            while !peek_op_chars.is_empty() {
                if let Ok(op) = Operator::from_str(&peek_op_chars) {
                    for _ in peek_op_chars.chars() {
                        self.next_char();
                    }
                    return Token {
                        kind: TokenKind::Operator(op),
                        loc: Span {
                            start,
                            end: self.current_loc.front(),
                        },
                    };
                }
                peek_op_chars.pop();
            }

            Token {
                kind: TokenKind::Unknown,
                loc: start.into(),
            }
        } else if let Some(Ok(delim)) = {
            self.peek_char(0)
                .map(|c| Delimiter::from_str(&c.to_string()))
        } {
            self.next_char();
            Token {
                kind: TokenKind::Delimiter(delim),
                loc: start.into(),
            }
        } else if self.is_peek_char_f(0, |c| char::is_alphabetic(c) || c == '_') {
            let ident = self
                .get_str_while(|c| char::is_alphanumeric(c) || c == '_')
                .unwrap();

            if let Ok(kw) = Keyword::from_str(&ident) {
                Token {
                    kind: TokenKind::Keyword(kw),
                    loc: Span {
                        start,
                        end: self.current_loc.front(),
                    },
                }
            } else if let Ok(kw) = TypeKeyword::from_str(&ident) {
                Token {
                    kind: TokenKind::TypeKeyword(kw),
                    loc: Span {
                        start,
                        end: self.current_loc.front(),
                    },
                }
            } else if let Ok(kw) = LiteralKeyword::from_str(&ident) {
                Token {
                    kind: TokenKind::LiteralKeyword(kw),
                    loc: Span {
                        start,
                        end: self.current_loc.front(),
                    },
                }
            } else {
                Token {
                    kind: TokenKind::Identifier(Identifier(ident)),
                    loc: Span {
                        start,
                        end: self.current_loc.front(),
                    },
                }
            }
        } else {
            self.next_char();
            Token {
                kind: TokenKind::Unknown,
                loc: start.into(),
            }
        }
    }

    fn skip_comment_and_whitespace(&mut self) {
        loop {
            let loc: Location = self.current_loc;
            self.skip_while(char::is_whitespace);

            if self.match_str("//") {
                self.skip_while(|c| c != '\n');
            }

            if self.match_str("/*") {
                while !self.match_str("*/") {
                    self.next_char();
                }
            }

            if self.current_loc == loc {
                break;
            }
        }
    }

    fn skip_while<F>(&mut self, cond: F)
    where
        F: Fn(char) -> bool,
    {
        while self.is_peek_char_f(0, &cond) {
            self.next_char();
        }
    }

    fn peek_str_while<F>(&mut self, cond: F) -> Option<String>
    where
        F: Fn(char) -> bool,
    {
        if !self.is_peek_char_f(0, &cond) {
            return None;
        }

        let mut value = String::new();
        while let Some(c) = self.peek_char(value.len()) {
            if cond(c) {
                value.push(c);
            } else {
                break;
            }
        }

        Some(value)
    }

    fn get_str_while<F>(&mut self, cond: F) -> Option<String>
    where
        F: Fn(char) -> bool,
    {
        if !self.is_peek_char_f(0, &cond) {
            return None;
        }

        let mut value = String::new();
        while self.is_peek_char_f(0, &cond) {
            value.push(self.next_char().unwrap());
        }

        Some(value)
    }

    fn match_str(&mut self, s: &str) -> bool {
        for (i, c) in s.chars().enumerate() {
            if !self.is_peek_char(i, c) {
                return false;
            }
        }
        for _ in 0..s.len() {
            self.next_char();
        }
        true
    }

    fn peek_char(&mut self, ahead: usize) -> Option<char> {
        self.source.chars().nth(self.index + ahead)
    }

    fn is_peek_char(&mut self, ahead: usize, c: char) -> bool {
        self.peek_char(ahead).map(|ch| ch == c).unwrap_or(false)
    }

    fn is_peek_char_f<F>(&mut self, ahead: usize, cond: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        self.peek_char(ahead).map(cond).unwrap_or(false)
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.peek_char(0)?;
        if c == '\n' {
            self.current_loc.line += 1;
            self.current_loc.col = 0;
        }
        self.current_loc.col += 1;
        self.index += 1;
        Some(c)
    }
}

macro_rules! match_token_kind {
    ($parser:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {{
        match $parser.lexer.peek_token_kind(0) {
            $pattern $(if $guard)? => Some($parser.lexer.next_token()),
            _ => None,
        }
    }};
}

macro_rules! match_identifier {
    ($parser:expr) => {{
        use $crate::ast::location::Located;
        use $crate::ast::parsing::token::{Identifier, Token, TokenKind};
        if let Some(Token {
            kind: TokenKind::Identifier(Identifier(ident)),
            loc,
        }) = match_token_kind!($parser, TokenKind::Identifier(_))
        {
            Some(Located(ident, loc))
        } else {
            None
        }
    }};
    ($parser:expr, $err_str:expr) => {{
        use $crate::ast::location::Located;
        match_identifier!($parser).ok_or_else(|| {
            Located(
                $crate::ast::parsing::ParseError::MissingIdentifier($err_str),
                $parser.lexer.get_prev_loc().next().into(),
            )
        })
    }};
    ($parser:expr, $err_str:expr =>) => {{
        match match_identifier!($parser, $err_str) {
            Ok(v) => Some(v),
            Err(e) => {
                $parser.push_error(e);
                None
            }
        }
    }};
}

macro_rules! match_unit_token_kind {
    ($parser:expr, $unit_kind:ident, $val:expr) => {{
        use $crate::ast::parsing::token::{Token, TokenKind};
        use $crate::ast::location::Located;
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

macro_rules! match_delimiter {
    ($parser:expr, $delim:expr) => {{
        use $crate::ast::location::Located;
        match_unit_token_kind!($parser, Delimiter, $delim.clone()).ok_or_else(|| {
            Located(
                $crate::ast::parsing::ParseError::MissingDelimiter($delim),
                $parser.lexer.get_prev_loc().next().into(),
            )
        })
    }};
    ($parser:expr, $delim:expr => $default:expr) => {{
        use $crate::ast::location::Located;
        match match_delimiter!($parser, $delim) {
            Ok(v) => v,
            Err(e) => {
                $parser.push_error(e);
                Located::new_temp($default)
            }
        }
    }};
    ($parser:expr, $delim:expr =>) => {{
        match_delimiter!($parser, $delim.clone() => $delim)
    }};
}

macro_rules! match_keyword {
    ($parser:expr, $kw:expr) => {{
        use $crate::ast::location::Located;
        match_unit_token_kind!($parser, Keyword, $kw.clone()).ok_or_else(|| {
            Located(
                $crate::ast::parsing::ParseError::MissingKeyword($kw),
                $parser.lexer.get_prev_loc().next().into(),
            )
        })
    }};
    ($parser:expr, $kw:expr => $default:expr) => {{
        use $crate::ast::location::Located;
        match match_keyword!($parser, $kw) {
            Ok(v) => v,
            Err(e) => {
                $parser.push_error(e);
                Located::new_temp($default)
            }
        }
    }};
    ($parser:expr, $kw:expr =>) => {{
        match_keyword!($parser, $kw.clone() => $kw)
    }};
}

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
                Located::new_temp($default)
            }
        }
    }};
    ($parser:expr, $op:expr =>) => {{
        match_operator!($parser, $op.clone() => $op)
    }};
}
