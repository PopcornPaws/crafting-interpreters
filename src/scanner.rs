use crate::token::{Token, Type as TokenType};
use thiserror::Error as ErrorT;

#[derive(ErrorT, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("invalid character ({character}) at line: {line} index: {index}")]
    InvalidCharacter {
        character: char,
        line: usize,
        index: usize,
    },
    #[error("missing \" to terminate literal")]
    UnterminatedLiteral,
}

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    errors: Vec<Error>,
    lines: usize,
}

impl<'a> Scanner<'a> {
    #[must_use]
    pub fn parse(source: &'a str) -> Self {
        let mut tokens = Vec::<Token>::new();
        let mut errors = Vec::<Error>::new();
        let mut lines = 1;

        let mut chars = source.char_indices().peekable();

        while chars.peek().is_some() {
            match dbg!(chars.next()) {
                Some((_, '(')) => tokens.push(Token::new(TokenType::LeftParent, lines)),
                Some((_, ')')) => tokens.push(Token::new(TokenType::RightParent, lines)),
                Some((_, '{')) => tokens.push(Token::new(TokenType::LeftBrace, lines)),
                Some((_, '}')) => tokens.push(Token::new(TokenType::RightBrace, lines)),
                Some((_, ',')) => tokens.push(Token::new(TokenType::Comma, lines)),
                Some((_, '.')) => tokens.push(Token::new(TokenType::Dot, lines)),
                Some((_, '-')) => tokens.push(Token::new(TokenType::Minus, lines)),
                Some((_, '+')) => tokens.push(Token::new(TokenType::Plus, lines)),
                Some((_, ';')) => tokens.push(Token::new(TokenType::Semicolon, lines)),
                Some((_, '*')) => tokens.push(Token::new(TokenType::Star, lines)),
                Some((_, '!')) => {
                    if let Some((_, '=')) = chars.peek() {
                        chars.next();
                        tokens.push(Token::new(TokenType::BangEqual, lines));
                    } else {
                        tokens.push(Token::new(TokenType::Bang, lines));
                    }
                }
                Some((_, '=')) => {
                    if let Some((_, '=')) = chars.peek() {
                        chars.next();
                        tokens.push(Token::new(TokenType::EqualEqual, lines));
                    } else {
                        tokens.push(Token::new(TokenType::Equal, lines));
                    }
                }
                Some((_, '<')) => {
                    if let Some((_, '=')) = chars.peek() {
                        chars.next();
                        tokens.push(Token::new(TokenType::LessEqual, lines));
                    } else {
                        tokens.push(Token::new(TokenType::Less, lines));
                    }
                }
                Some((_, '>')) => {
                    if let Some((_, '=')) = chars.peek() {
                        chars.next();
                        tokens.push(Token::new(TokenType::GreaterEqual, lines));
                    } else {
                        tokens.push(Token::new(TokenType::Greater, lines));
                    }
                }
                Some((_, '/')) => {
                    if let Some((_, '/')) = chars.peek() {
                        // consume lines because we encountered a comment
                        loop {
                            match chars.next() {
                                Some((_, '\n')) => {
                                    lines += 1;
                                    break;
                                }
                                None => break, // EOF reached
                                _ => {}
                            }
                        }
                    } else {
                        tokens.push(Token::new(TokenType::Slash, lines));
                    }
                }
                // string literal
                Some((start, '\"')) => loop {
                    match chars.next() {
                        Some((end, '\"')) => {
                            // strip quotes
                            tokens.push(Token::new(
                                TokenType::String(&source[start + 1..end]),
                                lines,
                            ));
                            break;
                        }
                        Some((_, '\n')) => lines += 1, // multilines string
                        None => {
                            errors.push(Error::UnterminatedLiteral);
                            break;
                        }
                        _ => {}
                    }
                },
                // add new line
                Some((_, '\n')) => lines += 1,
                // ignore whitespace, etc
                Some((_, ' ' | '\r' | '\t')) => {}
                // some invalid character
                Some((index, character)) => errors.push(Error::InvalidCharacter {
                    character,
                    line: lines,
                    index,
                }),
                None => unreachable!(), // because peek is_some
            }
        }

        Self {
            source,
            tokens,
            errors,
            lines,
        }
    }
}

impl Scanner<'_> {
    #[must_use]
    pub fn source(&self) -> &str {
        self.source
    }

    #[must_use]
    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    /*
    pub fn push(&mut self, typ: TokenType, literal: Option<String>) {
        let lexeme = &self.source[self.start..self.current];
        let token = Token {
            typ,
            lexeme,
            literal,
            lines: self.lines,
        };
        self.tokens.push(token);
    }
    */
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let input =
            "()\n\t// this is a comment\n(())(){} \r// grouping\n!*+-/=<> <= == // operators";

        let scanner = Scanner::parse(input);

        assert!(scanner.errors.is_empty());
        assert_eq!(scanner.source, input);
        assert_eq!(scanner.lines, 4);
        assert_eq!(
            scanner.tokens,
            &[
                Token::new(TokenType::LeftParent, 1),
                Token::new(TokenType::RightParent, 1),
                Token::new(TokenType::LeftParent, 3),
                Token::new(TokenType::LeftParent, 3),
                Token::new(TokenType::RightParent, 3),
                Token::new(TokenType::RightParent, 3),
                Token::new(TokenType::LeftParent, 3),
                Token::new(TokenType::RightParent, 3),
                Token::new(TokenType::LeftBrace, 3),
                Token::new(TokenType::RightBrace, 3),
                Token::new(TokenType::Bang, 4),
                Token::new(TokenType::Star, 4),
                Token::new(TokenType::Plus, 4),
                Token::new(TokenType::Minus, 4),
                Token::new(TokenType::Slash, 4),
                Token::new(TokenType::Equal, 4),
                Token::new(TokenType::Less, 4),
                Token::new(TokenType::Greater, 4),
                Token::new(TokenType::LessEqual, 4),
                Token::new(TokenType::EqualEqual, 4),
            ]
        );

        let scanner = Scanner::parse("\n~\n");
        assert!(scanner.tokens.is_empty());
        assert_eq!(scanner.lines, 3);
        assert_eq!(
            scanner.errors,
            &[Error::InvalidCharacter {
                character: '~',
                line: 2,
                index: 1,
            }]
        );
    }

    #[test]
    fn strings() {
        let scanner = Scanner::parse("\"\"");
        assert_eq!(scanner.tokens, &[Token::new(TokenType::String(""), 1)]);
        assert_eq!(scanner.lines, 1);

        let input = "\"hello world\"";
        let scanner = Scanner::parse(input);
        assert_eq!(
            scanner.tokens,
            &[Token::new(TokenType::String("hello world"), 1)]
        );

        let input = "\"hello\n\nworld\"";
        let scanner = Scanner::parse(input);
        assert_eq!(
            scanner.tokens,
            &[Token::new(TokenType::String("hello\n\nworld"), 3)]
        );
        assert_eq!(scanner.lines, 3);

        let input = "\"asd\n\r\t";
        let scanner = Scanner::parse(input);
        assert!(scanner.tokens.is_empty());
        assert_eq!(scanner.errors, &[Error::UnterminatedLiteral]);
    }
}
