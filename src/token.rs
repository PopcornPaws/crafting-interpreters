use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub typ: Type,
    pub lexeme: &'a str,
    pub literal: Option<String>, // TODO an object (another struct?)
    pub line: usize,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {:?}", self.typ, self.lexeme, self.literal)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Type {
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
    Identifier,
    String,
    Number,
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
    Eof,
}

impl TryFrom<&mut Peekable<Chars<'_>>> for Type {
    type Error = String;
    fn try_from(chars: &mut Peekable<Chars<'_>>) -> Result<Self, Self::Error> {
        match chars.next() {
            Some('(') => Ok(Self::LeftParent),
            Some(')') => Ok(Self::RightParent),
            Some('[') => Ok(Self::LeftBrace),
            Some(']') => Ok(Self::RightBrace),
            Some(',') => Ok(Self::Comma),
            Some('.') => Ok(Self::Dot),
            Some('-') => Ok(Self::Minus),
            Some('+') => Ok(Self::Plus),
            Some(';') => Ok(Self::Semicolon),
            Some('*') => Ok(Self::Star),
            Some('!') => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    Ok(Self::BangEqual)
                } else {
                    Ok(Self::Bang)
                }
            }
            Some('=') => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    Ok(Self::EqualEqual)
                } else {
                    Ok(Self::Equal)
                }
            }
            Some('<') => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    Ok(Self::LessEqual)
                } else {
                    Ok(Self::Less)
                }
            }
            Some('>') => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    Ok(Self::GreaterEqual)
                } else {
                    Ok(Self::Greater)
                }
            }
            Some('/') => {
                if let Some('/') = chars.peek() {
                    // consume line because we encountered a comment
                    while chars.next() != Some('\n') {}
                    // continue recoursively
                    Self::try_from(chars)
                } else {
                    Ok(Self::Greater)
                }
            }
            Some(unexpected) => Err(format!("unexpected character {unexpected:?}")),
            None => Err("EOF reached".to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        todo!();
    }
}
