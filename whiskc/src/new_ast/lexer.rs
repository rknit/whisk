use core::fmt;
use std::{io::Read, mem::size_of, str};

#[derive(Debug)]
pub struct Lexer<R: Read> {
    rd: CharReader<R>,
}
impl<R: Read> Lexer<R> {
    pub fn new(source: R) -> Self {
        Self {
            rd: CharReader::new(source),
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
