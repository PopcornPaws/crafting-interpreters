use crate::token::{Token, Type as TokenType};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    #[must_use]
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
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

    pub fn push(&mut self, typ: TokenType, literal: Option<String>) {
        let lexeme = &self.source[self.start..self.current];
        let token = Token {
            typ,
            lexeme,
            literal,
            line: self.line,
        };
        self.tokens.push(token);
    }
}
