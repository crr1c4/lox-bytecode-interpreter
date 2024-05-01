pub mod token;

#[cfg(test)]
pub mod tests;

use token::*;
use std::iter::Iterator;
// TODO: Change this file, implement Iterator trait for scanning on demand :)

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            line: 1,
            start: 0,
            current: 0,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        self.advance();

        if self.peek().unwrap().is_digit(10) {
            dbg!(self.peek());
            return self.make_number();
        } else if self.peek().unwrap().is_alphabetic() || self.peek().unwrap() == '_' {
            return self.make_identifier();
        }

        match &self.source[self.start..self.current] {
            "(" => self.make_token(TokenType::LeftParen),
            ")" => self.make_token(TokenType::RightParen),
            "{" => self.make_token(TokenType::LeftBrace),
            "}" => self.make_token(TokenType::RightBrace),
            ";" => self.make_token(TokenType::Semicolon),
            "," => self.make_token(TokenType::Comma),
            "." => self.make_token(TokenType::Dot),
            "-" => self.make_token(TokenType::Minus),
            "+" => self.make_token(TokenType::Plus),
            "/" => self.make_token(TokenType::Slash),
            "*" => self.make_token(TokenType::Star),
            // Two characters match
            "!" => match self.source.get(self.start..self.current + 1) {
                Some("!=") => {
                    self.advance();
                    self.make_token(TokenType::BangEqual)
                }
                _ => self.make_token(TokenType::Bang),
            },
            "=" => match self.source.get(self.start..self.current + 1) {
                Some("==") => {
                    self.advance();
                    self.make_token(TokenType::EqualEqual)
                }
                _ => self.make_token(TokenType::Equal),
            },
            ">" => match self.source.get(self.start..self.current + 1) {
                Some(">=") => {
                    self.advance();
                    self.make_token(TokenType::GreaterEqual)
                }
                _ => self.make_token(TokenType::Greater),
            },
            "<" => match self.source.get(self.start..self.current + 1) {
                Some("<=") => {
                    self.advance();
                    self.make_token(TokenType::LessEqual)
                }
                _ => self.make_token(TokenType::Less),
            },
            "\"" => self.make_string(),
            _ => self.make_error_token("Unexpected character"),
        }
    }

    fn make_identifier(&mut self) -> Token {
        while self.peek().unwrap().is_alphabetic() || self.peek().unwrap().is_digit(10) {
            self.advance()
        }

        // self.make_token(identifier_type())
        self.make_token(self.choose_identifier_type())
    }

    fn choose_identifier_type(&self) -> TokenType {
        match &self.source[self.start..self.current] {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "super" => TokenType::Super,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "this" => TokenType::This,
            "true" => TokenType::True,
            _ => TokenType::Identifier,
        }
    }

    fn make_number(&mut self) -> Token {
        while let Some(c) = self.peek() {
            match c {
                c if c.is_digit(10) => self.advance(),
                _ => break,
            }
        }

        if self.peek().unwrap() == '.' && self.peek_next().unwrap().is_digit(10) {
            self.advance();
        }

        while let Some(c) = self.peek() {
            match c {
                c if c.is_digit(10) => self.advance(),
                _ => break,
            }
        }

        self.make_token(TokenType::Number)
    }

    fn make_string(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if c == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.make_error_token("Unterminated string.");
        }

        self.advance();
        self.make_token(TokenType::String)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                ' ' | '\r' | '\t' => self.advance(),
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' if self.peek_next().is_some() && self.peek_next().unwrap() == '/' => {
                    while let Some(c) = self.peek() {
                        if c == '\n' {
                            return;
                        } else {
                            self.advance();
                        }
                    }
                }
                _ => return,
            }
        }
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        if self.source.chars().nth(self.current + 1).is_some() {
            self.source.chars().nth(self.current + 1)
        } else {
            Some('\0')
        }
    }

    fn is_at_end(&self) -> bool {
        self.source.chars().nth(self.current as usize).is_none()
    }

    fn make_token(&self, kind: TokenType) -> Token {
        Token::new(kind, &self.source[self.start..self.current], self.line)
    }

    fn make_error_token<'a>(&'a self, message: &'a str) -> Token {
        Token::new(TokenType::ERROR, message, self.line)
    }
}
