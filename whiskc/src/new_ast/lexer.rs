use core::fmt;
use std::{
    collections::VecDeque,
    io::Read,
    mem::size_of,
    str::{self, FromStr},
};

use crate::ast::location::{Location, Span};

use super::token::{Delimiter, Keyword, Literal, Token, TokenKind, TypeKeyword};

#[derive(Debug)]
pub struct Lexer<R: Read> {
    rd: CharReader<R>,
    buf: VecDeque<Char>,
    toks: VecDeque<Token>,
    pos: Location,
}
impl<R: Read> Lexer<R> {
    pub fn new(source: R) -> Self {
        Self {
            rd: CharReader::new(source),
            buf: VecDeque::new(),
            toks: VecDeque::new(),
            pos: Location::new(1, 1),
        }
    }

    pub fn is_eof(&self) -> bool {
        self.rd.is_eof() && (self.buf.is_empty() || matches!(self.buf.front(), Some(Char::EOF)))
    }

    pub fn peek_kind(&mut self) -> &TokenKind {
        self.peek_kind_ahead(0)
    }

    pub fn peek_kind_ahead(&mut self, ahead: usize) -> &TokenKind {
        &self.peek_token_ahead(ahead).kind
    }

    pub fn peek_token(&mut self) -> &Token {
        self.peek_token_ahead(0)
    }

    pub fn peek_token_ahead(&mut self, ahead: usize) -> &Token {
        self.ensure_token_count(ahead + 1);
        unsafe { self.toks.get(ahead).unwrap_unchecked() }
    }

    pub fn next_token(&mut self) -> Token {
        self.ensure_token_count(1);
        unsafe { self.toks.pop_front().unwrap_unchecked() }
    }

    fn ensure_token_count(&mut self, n: usize) {
        while self.toks.len() < n {
            self.make_token();
        }
    }

    fn make_token(&mut self) {
        self.skip_whitespace_and_comment();
        let start = self.pos;

        let token = if self.is_eof() {
            Token::new(TokenKind::EndOfFile, start)
        } else if self.peek_char().is(|c| char::is_ascii_digit(&c)) {
            let int = self.match_while(|c| char::is_ascii_digit(&c));
            let int = int.parse::<i64>().expect("valid 64 bit decimal integer");
            Token::new(Literal::Int(int), Span::new(start, self.pos.front()))
        } else if self
            .peek_char()
            .is(|c| Delimiter::from_str(c.to_string().as_str()).is_ok())
        {
            let Char::Char(c) = self.next_char() else {
                unreachable!()
            };
            let delim = unsafe { Delimiter::from_str(c.to_string().as_str()).unwrap_unchecked() };
            Token::new(delim, Span::new(start, self.pos.front()))
        } else if self.peek_char().is(|c| c.is_ascii_alphabetic() || c == '_') {
            let ident = self.match_while(|c| c.is_ascii_alphanumeric() || c == '_');
            let span = Span::new(start, self.pos.front());
            if let Ok(kw) = Keyword::from_str(&ident) {
                Token::new(kw, span)
            } else if let Ok(kw) = TypeKeyword::from_str(&ident) {
                Token::new(kw, span)
            } else {
                Token::new(ident, span)
            }
        } else {
            let c = self.next_char();
            Token::new(TokenKind::Unknown(c.to_string()), start)
        };

        self.toks.push_back(token);
    }

    pub fn skip_whitespace_and_comment(&mut self) {
        loop {
            let start = self.pos;

            self.skip_while(char::is_whitespace);

            if self.match_string("//") {
                self.skip_while(|c| c != '\n');
                self.skip();
            }

            if self.match_string("/*") {
                while !self.match_string("*/") {
                    self.skip();
                }
            }

            if start == self.pos {
                break;
            }
        }
    }

    pub fn peek_while(&mut self, cond: impl Fn(char) -> bool) -> String {
        let mut s = String::new();
        loop {
            self.ensure_char_buf_len(s.len() + 1);
            let Some(Char::Char(c)) = self.buf.get(s.len()) else {
                break;
            };
            if !cond(*c) {
                break;
            }
            s.push(*c);
        }
        s
    }

    pub fn match_while(&mut self, cond: impl Fn(char) -> bool) -> String {
        let s = self.peek_while(cond);
        for _ in 0..s.len() {
            self.skip();
        }
        s
    }

    pub fn peek_string(&mut self, s: &str) -> bool {
        self.ensure_char_buf_len(s.len());
        for (ch, c) in self.buf.iter().zip(s.chars()) {
            if !matches!(ch, Char::Char(ch) if *ch == c) {
                return false;
            }
        }
        true
    }

    pub fn match_string(&mut self, s: &str) -> bool {
        if self.peek_string(s) {
            for _ in 0..s.len() {
                self.skip();
            }
            true
        } else {
            false
        }
    }

    pub fn peek_char(&mut self) -> &Char {
        self.peek_char_ahead(0)
    }

    pub fn peek_char_ahead(&mut self, ahead: usize) -> &Char {
        self.ensure_char_buf_len(ahead + 1);
        unsafe { self.buf.get(ahead).unwrap_unchecked() }
    }

    pub fn skip_while(&mut self, cond: impl Fn(char) -> bool) {
        while self.peek_char().is(&cond) {
            self.skip();
        }
    }

    pub fn skip(&mut self) {
        _ = self.next_char();
    }

    #[must_use]
    pub fn next_char(&mut self) -> Char {
        self.ensure_char_buf_len(1);

        let c = unsafe { self.buf.pop_front().unwrap_unchecked() };
        if matches!(c, Char::EOF) {
            return c;
        }

        if matches!(c, Char::Char('\n')) {
            self.pos.line += 1;
            self.pos.col = 0;
        }
        self.pos.col += 1;

        c
    }

    fn ensure_char_buf_len(&mut self, n: usize) {
        if self.buf.len() >= n {
            return;
        }
        for _ in 0..n {
            let c = self.rd.next_char();
            self.buf.push_back(c);
        }
    }
}

const BUFFER_SIZE: usize = MIN_BUFFER_SIZE * 512;
const MIN_BUFFER_SIZE: usize = size_of::<char>();

#[derive(Debug)]
pub enum Char {
    EOF,
    Invalid,
    Char(char),
}
impl Char {
    pub fn is(&self, f: impl FnOnce(char) -> bool) -> bool {
        let Self::Char(c) = self else {
            return false;
        };
        f(*c)
    }
}
impl fmt::Display for Char {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Char::EOF => "<EOF>".to_owned(),
                Char::Invalid => "<Invalid>".to_owned(),
                Char::Char(c) => c.to_string(),
            }
        )
    }
}

#[derive(Debug)]
pub struct CharReader<R: Read> {
    rd: R,
    buf: [u8; BUFFER_SIZE],
    index: usize,
    end: usize,
    valid_end: usize,
    eof: bool,
}
impl<R: Read> CharReader<R> {
    pub fn new(source: R) -> Self {
        Self {
            rd: source,
            buf: [0; BUFFER_SIZE],
            index: 0,
            end: 0,
            valid_end: 0,
            eof: false,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.eof && self.get_buffer_len() == 0
    }

    pub fn next_char(&mut self) -> Char {
        if self.is_eof() {
            Char::EOF
        } else if self.get_buffer_len() == 0 {
            if self.fill_buffer().is_none() {
                Char::EOF
            } else {
                self.next_char()
            }
        } else if self.get_valid_buffer_len() > 0 {
            let ch = unsafe {
                let s = str::from_utf8(&self.buf[self.index..self.valid_end]).unwrap_unchecked();
                s.chars().next().unwrap_unchecked()
            };
            self.index += ch.len_utf8();
            Char::Char(ch)
        } else {
            match str::from_utf8(&self.buf[self.index..self.end]) {
                Ok(_) => {
                    self.valid_end = self.end;
                    self.next_char()
                }
                Err(e) => {
                    if e.valid_up_to() > 0 {
                        self.valid_end = self.index + e.valid_up_to();
                        self.next_char()
                    } else {
                        match e.error_len() {
                            Some(n) => {
                                self.index += n;
                                Char::Invalid
                            }
                            None => match self.fill_buffer() {
                                None => Char::Invalid,
                                _ => self.next_char(),
                            },
                        }
                    }
                }
            }
        }
    }

    fn get_valid_buffer_len(&self) -> usize {
        self.valid_end - self.index
    }

    fn get_buffer_len(&self) -> usize {
        self.end - self.index
    }

    fn fill_buffer(&mut self) -> Option<usize> {
        if self.eof {
            return None;
        }
        for i in self.index..self.end {
            self.buf[i - self.index] = self.buf[i];
        }
        self.valid_end -= self.index;
        self.end -= self.index;
        self.index = 0;

        match self.rd.read(&mut self.buf[self.end..]) {
            Ok(sz) => {
                if sz < self.buf.len() - self.end + 1 {
                    self.eof = true;
                }
                self.end += sz;
                Some(sz)
            }
            Err(_) => {
                self.eof = true;
                None
            }
        }
    }
}
