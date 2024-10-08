#[allow(clippy::wildcard_imports)]
use crate::consts::*;
use crate::token::{Token, Type as TokenType, KEYWORDS};
use thiserror::Error as ErrorT;

use std::num::ParseFloatError;
use std::str::FromStr;

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
    #[error(transparent)]
    InvalidNumber(#[from] ParseFloatError),
}

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    errors: Vec<Error>,
    lines: usize,
}

impl<'a> Scanner<'a> {
    #[must_use]
    #[allow(clippy::too_many_lines)] // TODO terrible spaghetti
    pub fn parse(source: &'a str) -> Self {
        let mut tokens = Vec::<Token>::new();
        let mut errors = Vec::<Error>::new();
        let mut lines = 1;

        let mut chars = source.char_indices().peekable();

        while chars.peek().is_some() {
            match chars.next() {
                Some((_, LEFT_PARENT)) => tokens.push(Token::new(TokenType::LeftParent, lines)),
                Some((_, RIGHT_PARENT)) => tokens.push(Token::new(TokenType::RightParent, lines)),
                Some((_, LEFT_BRACE)) => tokens.push(Token::new(TokenType::LeftBrace, lines)),
                Some((_, RIGHT_BRACE)) => tokens.push(Token::new(TokenType::RightBrace, lines)),
                Some((_, COMMA)) => tokens.push(Token::new(TokenType::Comma, lines)),
                Some((_, DOT)) => tokens.push(Token::new(TokenType::Dot, lines)),
                Some((_, MINUS)) => tokens.push(Token::new(TokenType::Minus, lines)),
                Some((_, PLUS)) => tokens.push(Token::new(TokenType::Plus, lines)),
                Some((_, SEMICOLON)) => tokens.push(Token::new(TokenType::Semicolon, lines)),
                Some((_, STAR)) => tokens.push(Token::new(TokenType::Star, lines)),
                Some((_, BANG)) => {
                    if let Some((_, EQUAL)) = chars.peek() {
                        chars.next();
                        tokens.push(Token::new(TokenType::BangEqual, lines));
                    } else {
                        tokens.push(Token::new(TokenType::Bang, lines));
                    }
                }
                Some((_, EQUAL)) => {
                    if let Some((_, EQUAL)) = chars.peek() {
                        chars.next();
                        tokens.push(Token::new(TokenType::EqualEqual, lines));
                    } else {
                        tokens.push(Token::new(TokenType::Equal, lines));
                    }
                }
                Some((_, LESS)) => {
                    if let Some((_, EQUAL)) = chars.peek() {
                        chars.next();
                        tokens.push(Token::new(TokenType::LessEqual, lines));
                    } else {
                        tokens.push(Token::new(TokenType::Less, lines));
                    }
                }
                Some((_, GREATER)) => {
                    if let Some((_, EQUAL)) = chars.peek() {
                        chars.next();
                        tokens.push(Token::new(TokenType::GreaterEqual, lines));
                    } else {
                        tokens.push(Token::new(TokenType::Greater, lines));
                    }
                }
                Some((_, SLASH)) => {
                    if let Some((_, SLASH)) = chars.peek() {
                        // consume lines because we encountered a comment
                        // TODO could we use while chars.next_if(|&c| c != '\n').is_some()
                        // or we can't do this because we only want to increment lines if
                        // '\n' is encountered
                        loop {
                            match chars.next() {
                                Some((_, NEWLINE)) => {
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
                Some((start, QUOTE)) => loop {
                    match chars.next() {
                        Some((end, QUOTE)) => {
                            // strip quotes
                            tokens.push(Token::new(
                                TokenType::String(&source[start + 1..end]),
                                lines,
                            ));
                            break;
                        }
                        Some((_, NEWLINE)) => lines += 1, // multilines string
                        None => {
                            errors.push(Error::UnterminatedLiteral);
                            break;
                        }
                        _ => {}
                    }
                },
                // add new line
                Some((_, NEWLINE)) => lines += 1,
                // ignore whitespace, etc
                Some((_, WHITESPACE | CARRIAGE_RETURN | TAB)) => {}
                // some invalid character
                Some((start, character)) => {
                    // try parsing a number
                    if character.is_numeric() {
                        // consume number until '.'
                        let mut end = start;
                        while let Some((e, _num)) = chars.next_if(|(_, n)| n.is_numeric()) {
                            end = e;
                        }

                        if let (Some(dot), Some(num)) = (
                            source.get(end + 1..end + 2),
                            source
                                .get(end + 2..end + 3)
                                .and_then(|slice| slice.chars().next()),
                        ) {
                            if dot == "." && num.is_numeric() {
                                // consume the dot and parse the fractional part
                                chars.next();
                                while let Some((e, _num)) = chars.next_if(|(_, n)| n.is_numeric()) {
                                    end = e;
                                }
                            }
                        }

                        let num_literal = &source[start..=end];
                        match f32::from_str(num_literal) {
                            Ok(parsed_f32) => {
                                tokens.push(Token::new(TokenType::Number(parsed_f32), lines));
                            }
                            Err(error) => errors.push(error.into()),
                        }
                    // try parsing an identifier
                    } else if character.is_alphabetic() {
                        // consume until chars are alphanumeric
                        let mut end = start;
                        while let Some((e, _num)) = chars.next_if(|(_, n)| n.is_alphanumeric()) {
                            end = e;
                        }
                        let identifier = &source[start..=end];
                        let token = if let Some(typ) = KEYWORDS.get(identifier) {
                            Token::new(typ.clone(), lines) // keyword types are cheap to clone
                        } else {
                            Token::new(TokenType::Identifier(identifier), lines)
                            // keyword types are cheap to clone
                        };
                        tokens.push(token);
                    } else {
                        errors.push(Error::InvalidCharacter {
                            character,
                            line: lines,
                            index: start,
                        });
                    }
                }
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

    #[must_use]
    pub fn errors(&self) -> &[Error] {
        &self.errors
    }

    #[must_use]
    pub fn lines(&self) -> usize {
        self.lines
    }
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

    #[test]
    fn numbers() {
        let scanner = Scanner::parse("1");
        assert_eq!(scanner.tokens, &[Token::new(TokenType::Number(1.0), 1)]);
        let scanner = Scanner::parse("1234");
        assert_eq!(scanner.tokens, &[Token::new(TokenType::Number(1234.0), 1)]);
        let scanner = Scanner::parse("1234.");
        assert_eq!(
            scanner.tokens,
            &[
                Token::new(TokenType::Number(1234.0), 1),
                Token::new(TokenType::Dot, 1)
            ]
        );
        let scanner = Scanner::parse("1234.5678");
        assert_eq!(
            scanner.tokens,
            &[Token::new(TokenType::Number(1234.5678), 1)]
        );
        let scanner = Scanner::parse("12.34.56.78");
        assert_eq!(
            scanner.tokens,
            &[
                Token::new(TokenType::Number(12.34), 1),
                Token::new(TokenType::Dot, 1),
                Token::new(TokenType::Number(56.78), 1),
            ]
        );
    }

    #[test]
    fn identifiers() {
        let scanner = Scanner::parse("a");
        assert_eq!(scanner.tokens, &[Token::new(TokenType::Identifier("a"), 1)]);
        let scanner = Scanner::parse("and");
        assert_eq!(scanner.tokens, &[Token::new(TokenType::And, 1)]);
        let scanner = Scanner::parse("and;\n nil;\n a0123.sqrt();");
        assert_eq!(
            scanner.tokens,
            &[
                Token::new(TokenType::And, 1),
                Token::new(TokenType::Semicolon, 1),
                Token::new(TokenType::Nil, 2),
                Token::new(TokenType::Semicolon, 2),
                Token::new(TokenType::Identifier("a0123"), 3),
                Token::new(TokenType::Dot, 3),
                Token::new(TokenType::Identifier("sqrt"), 3),
                Token::new(TokenType::LeftParent, 3),
                Token::new(TokenType::RightParent, 3),
                Token::new(TokenType::Semicolon, 3),
            ]
        );
    }
}
