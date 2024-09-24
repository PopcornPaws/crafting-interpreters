#[allow(clippy::wildcard_imports)]
use crate::consts::*;
use lazy_static::lazy_static;

use std::collections::HashMap;
use std::fmt;

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, Type<'static>> = vec![
        (AND, Type::And),
        (CLASS, Type::Class),
        (ELSE, Type::Else),
        (FALSE, Type::False),
        (FUN, Type::Fun),
        (FOR, Type::For),
        (IF, Type::If),
        (NIL, Type::Nil),
        (OR, Type::Or),
        (PRINT, Type::Print),
        (RETURN, Type::Return),
        (SUPER, Type::Super),
        (THIS, Type::This),
        (TRUE, Type::True),
        (VAR, Type::Var),
        (WHILE, Type::While),
    ]
    .into_iter()
    .collect();
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token<'a> {
    pub typ: Type<'a>,
    pub line: usize,
}

impl<'a> Token<'a> {
    #[must_use]
    pub fn new(typ: Type<'a>, line: usize) -> Self {
        Self { typ, line }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type<'a> {
    // Single character tokens
    LeftParent,
    RightParent,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier(&'a str),
    String(&'a str),
    Number(f32),
    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

impl fmt::Display for Type<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LeftParent => write!(f, "{LEFT_PARENT}"),
            Self::RightParent => write!(f, "{RIGHT_PARENT}"),
            Self::LeftBrace => write!(f, "{LEFT_BRACE}"),
            Self::RightBrace => write!(f, "{RIGHT_BRACE}"),
            Self::Comma => write!(f, "{COMMA}"),
            Self::Dot => write!(f, "{DOT}"),
            Self::Minus => write!(f, "{MINUS}"),
            Self::Plus => write!(f, "{PLUS}"),
            Self::Semicolon => write!(f, "{SEMICOLON}"),
            Self::Slash => write!(f, "{SLASH}"),
            Self::Star => write!(f, "{STAR}"),
            Self::Bang => write!(f, "{BANG}"),
            Self::BangEqual => write!(f, "{BANG_EQUAL}"),
            Self::Equal => write!(f, "{EQUAL}"),
            Self::EqualEqual => write!(f, "{EQUAL_EQUAL}"),
            Self::Greater => write!(f, "{GREATER}"),
            Self::GreaterEqual => write!(f, "{GREATER_EQUAL}"),
            Self::Less => write!(f, "{LESS}"),
            Self::LessEqual => write!(f, "{LESS_EQUAL}"),
            Self::Identifier(inner) | Self::String(inner) => {
                write!(f, "{inner}")
            }
            Self::Number(num) => write!(f, "{num}"),
            Self::And => write!(f, "{AND}"),
            Self::Class => write!(f, "{CLASS}"),
            Self::Else => write!(f, "{ELSE}"),
            Self::False => write!(f, "{FALSE}"),
            Self::Fun => write!(f, "{FUN}"),
            Self::For => write!(f, "{FOR}"),
            Self::If => write!(f, "{IF}"),
            Self::Nil => write!(f, "{NIL}"),
            Self::Or => write!(f, "{OR}"),
            Self::Print => write!(f, "{PRINT}"),
            Self::Return => write!(f, "{RETURN}"),
            Self::Super => write!(f, "{SUPER}"),
            Self::This => write!(f, "{THIS}"),
            Self::True => write!(f, "{TRUE}"),
            Self::Var => write!(f, "{VAR}"),
            Self::While => write!(f, "{WHILE}"),
        }
    }
}
