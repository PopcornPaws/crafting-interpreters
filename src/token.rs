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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    // Misc
    Newline,
    Eof,
}

impl TryFrom<&mut Peekable<Chars<'_>>> for Type {
    type Error = String;
    fn try_from(chars: &mut Peekable<Chars<'_>>) -> Result<Self, Self::Error> {
        match chars.next() {
            Some('(') => Ok(Self::LeftParent),
            Some(')') => Ok(Self::RightParent),
            Some('{') => Ok(Self::LeftBrace),
            Some('}') => Ok(Self::RightBrace),
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
                    loop {
                        let next = chars.next();
                        if next.is_none() {
                            return Ok(Self::Eof);
                        } else if next == Some('\n') {
                            return Ok(Self::Newline);
                        }
                    }
                } else {
                    Ok(Self::Greater)
                }
            }
            // ignore whitespace, etc, continue recursively
            Some(' ' | '\r' | '\t') => Self::try_from(chars),
            Some('\n') => Ok(Self::Newline),
            Some(unexpected) => Err(format!("unexpected character {unexpected:?}")),
            None => Ok(Self::Eof),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let input =
            "()\n\t// this is a comment\n(())(){} \r// grouping\n!*+-/=<> <= == // operators";

        let mut chars = input.chars().peekable();
        let mut types = Vec::<Type>::new();
        let mut lines = 1_usize;
        while chars.peek().is_some() {
            let ty = Type::try_from(&mut chars).unwrap();
            if let Type::Newline = ty {
                lines += 1;
            }
            types.push(ty);
        }
        assert_eq!(lines, 4);
        assert_eq!(types, vec![]);
    }
}
