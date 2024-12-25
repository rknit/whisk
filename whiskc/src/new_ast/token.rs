use core::fmt;
use std::hash::Hash;
use std::mem::discriminant;
use std::str::FromStr;

use strum::EnumIter;
use strum::IntoEnumIterator;

use crate::ast::location::LocationRange;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub loc: LocationRange,
}
impl Token {
    pub fn new(kind: impl Into<TokenKind>, loc: impl Into<LocationRange>) -> Self {
        Self {
            kind: kind.into(),
            loc: loc.into(),
        }
    }

    pub fn temp(kind: impl Into<TokenKind>) -> Self {
        Self {
            kind: kind.into(),
            loc: LocationRange::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Unknown(String),
    EndOfFile,
    Literal(Literal),
    Keyword(Keyword),
    LiteralKeyword(LiteralKeyword),
    TypeKeyword(TypeKeyword),
    Identifier(String),
    Delimiter(Delimiter),
    Operator(Operator),
}
impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unknown(s) => format!("UNK('{}')", s),
                Self::EndOfFile => "EOF".to_owned(),
                Self::Literal(lit) => format!("{}", lit),
                Self::LiteralKeyword(lit) => format!("{}", lit),
                Self::Keyword(kw) => format!("'{}' keyword", kw),
                Self::TypeKeyword(kw) => format!("'{}' type", kw),
                Self::Identifier(ident) => ident.clone(),
                Self::Delimiter(delim) => format!("'{}'", delim),
                Self::Operator(op) => format!("'{}' operator", op),
            }
        )
    }
}

#[derive(Debug, Clone, Copy, Eq)]
pub enum Literal {
    Int(i64),
}
impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}
impl fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Int(i) => i.to_string(),
            }
        )
    }
}

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LiteralKeyword {
    True,
    False,
}
impl fmt::Display for LiteralKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::True => "true",
                Self::False => "false",
            }
        )
    }
}
impl FromStr for LiteralKeyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for kw in Self::iter() {
            if kw.to_string() == s {
                return Ok(kw);
            }
        }
        Err(())
    }
}

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyword {
    Pub,
    Extern,
    Func,
    Let,
    If,
    Else,
    Return,
    As,
    Loop,
}
impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pub => "pub",
                Self::Extern => "extern",
                Self::Func => "func",
                Self::Let => "let",
                Self::If => "if",
                Self::Else => "else",
                Self::Return => "return",
                Self::As => "as",
                Self::Loop => "loop",
            }
        )
    }
}
impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for kw in Self::iter() {
            if kw.to_string() == s {
                return Ok(kw);
            }
        }
        Err(())
    }
}

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeKeyword {
    Bool,
    Int,
}
impl fmt::Display for TypeKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bool => "bool",
                Self::Int => "int",
            }
        )
    }
}
impl FromStr for TypeKeyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for kw in Self::iter() {
            if kw.to_string() == s {
                return Ok(kw);
            }
        }
        Err(())
    }
}

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Delimiter {
    ParenOpen,
    ParenClose,
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    Colon,
    Semicolon,
    Comma,
}
impl fmt::Display for Delimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ParenOpen => '(',
                Self::ParenClose => ')',
                Self::BraceOpen => '{',
                Self::BraceClose => '}',
                Self::BracketOpen => '[',
                Self::BracketClose => ']',
                Self::Colon => ':',
                Self::Semicolon => ';',
                Self::Comma => ',',
            }
        )
    }
}
impl FromStr for Delimiter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for v in Self::iter() {
            if v.to_string() == s {
                return Ok(v);
            }
        }
        Err(())
    }
}

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperatorChar {
    Equal,
    Plus,
    Minus,
    Less,
    Greater,
    Exclaimation,
    Ampersand,
    Pipe,
}
impl fmt::Display for OperatorChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Equal => '=',
                Self::Plus => '+',
                Self::Minus => '-',
                Self::Less => '<',
                Self::Greater => '>',
                Self::Exclaimation => '!',
                Self::Ampersand => '&',
                Self::Pipe => '|',
            }
        )
    }
}
impl FromStr for OperatorChar {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for v in Self::iter() {
            if v.to_string() == s {
                return Ok(v);
            }
        }
        Err(())
    }
}

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    Assign,
    Add,
    Sub,
    And,
    Or,
    Not,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}
impl fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Assign => "=",
                Self::Add => "+",
                Self::Sub => "-",
                Self::And => "&&",
                Self::Or => "||",
                Self::Not => "!",
                Self::Equal => "==",
                Self::NotEqual => "!=",
                Self::Less => "<",
                Self::LessEqual => "<=",
                Self::Greater => ">",
                Self::GreaterEqual => ">=",
            }
        )
    }
}
impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for v in Self::iter() {
            if v.to_string() == s {
                return Ok(v);
            }
        }
        Err(())
    }
}
