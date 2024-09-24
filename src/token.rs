use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, Type<'static>> = vec![
        ("and", Type::And),
        ("class", Type::Class),
        ("else", Type::Else),
        ("false", Type::False),
        ("fun", Type::Fun),
        ("for", Type::For),
        ("if", Type::If),
        ("nil", Type::Nil),
        ("or", Type::Or),
        ("print", Type::Print),
        ("return", Type::Return),
        ("super", Type::Super),
        ("this", Type::This),
        ("true", Type::True),
        ("var", Type::Var),
        ("while", Type::While),
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
