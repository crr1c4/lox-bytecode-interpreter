pub mod token;

use token::*;

#[derive(Debug)]
pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: u32,
    eof_reached: bool,
}

impl Iterator for Scanner {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        self.start = self.current;

        if self.eof_reached {
            return None;
        }

        if self.is_at_end() {
            self.eof_reached = true;
            return Some(self.make_token(TokenKind::EOF));
        }

        self.current += 1;

        Some(match (self.peek(self.current - 1), self.peek(self.current)) {
            // Special cases
            (Some('.'), Some(digit)) if digit.is_ascii_digit() => self.make_number(),
            (Some('"'), _) => self.make_string(),
            (Some(digit), _) if digit.is_ascii_digit() => self.make_number(),
            (Some(character), _) if character.is_alphabetic() || character == '_' => self.make_identifier_or_keyword(),
            // Single character
            (Some('('), _) => self.make_token(TokenKind::LeftParen),
            (Some(')'), _) => self.make_token(TokenKind::RightParen),
            (Some('{'), _) => self.make_token(TokenKind::LeftBrace),
            (Some('}'), _) => self.make_token(TokenKind::RightBrace),
            (Some(';'), _) => self.make_token(TokenKind::Semicolon),
            (Some(','), _) => self.make_token(TokenKind::Comma),
            (Some('.'), _) => self.make_token(TokenKind::Dot),
            (Some('-'), _) => self.make_token(TokenKind::Minus),
            (Some('+'), _) => self.make_token(TokenKind::Plus),
            (Some('/'), _) => self.make_token(TokenKind::Slash),
            (Some('*'), _) => self.make_token(TokenKind::Star),
            // Two characters match
            (Some('!'), Some('=')) => self.make_token(TokenKind::BangEqual),
            (Some('!'), _) => self.make_token(TokenKind::Bang),
            (Some('='), Some('=')) => self.make_token(TokenKind::EqualEqual),
            (Some('='), _) => self.make_token(TokenKind::Equal),
            (Some('<'), Some('=')) => self.make_token(TokenKind::LessEqual),
            (Some('<'), _) => self.make_token(TokenKind::Less),
            (Some('>'), Some('=')) => self.make_token(TokenKind::GreaterEqual),
            (Some('>'), _) => self.make_token(TokenKind::Greater),
            _ => self.make_error_token("Unexpected character"),
        })
    }
}

impl Scanner {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            line: 1,
            start: 0,
            current: 0,
            eof_reached: false,
        }
    }

    fn make_identifier_or_keyword(&mut self) -> Token {
        while let Some(c) = self.peek(self.current) {
            if c.is_alphanumeric() || c == '_' {
                self.current += 1;
            } else {
                break;
            }
        }

        let kind = match self.source.get(self.start..self.current) {
            Some("and") => TokenKind::And,
            Some("class") => TokenKind::Class,
            Some("else") => TokenKind::Else,
            Some("if") => TokenKind::If,
            Some("nil") => TokenKind::Nil,
            Some("or") => TokenKind::Or,
            Some("print") => TokenKind::Print,
            Some("super") => TokenKind::Super,
            Some("var") => TokenKind::Var,
            Some("while") => TokenKind::While,
            Some("false") => TokenKind::False,
            Some("for") => TokenKind::For,
            Some("fun") => TokenKind::Fun,
            Some("this") => TokenKind::This,
            Some("true") => TokenKind::True,
            _ => TokenKind::Identifier,
        };

        self.make_token(kind)
    }

    fn make_number(&mut self) -> Token {
        while let Some(possible_digit) = self.peek(self.current) {
            if possible_digit.is_ascii_digit() {
                self.current += 1;
            } else {
                break;
            }
        }

        match (self.peek(self.current), self.peek(self.current + 1)) {
            (Some('.'), Some(possible_digit)) if possible_digit.is_ascii_digit() => self.current += 1,
            _ => (),
        }

        while let Some(possible_digit) = self.peek(self.current) {
            if possible_digit.is_ascii_digit() {
                self.current += 1;
            } else {
                break;
            }
        }

        self.make_token(TokenKind::Number)
    }

    fn make_string(&mut self) -> Token {
        while let Some(character) = self.peek(self.current) {
            match character {
                '"' => break,
                '\n' => self.line += 1,
                _ => self.current += 1,
            }
        }

        if self.is_at_end() {
            return self.make_error_token("Unterminated string.");
        }

        // NOTE: The closing quote.
        self.current += 1;
        self.make_token(TokenKind::String)
    }

    fn skip_whitespace(&mut self) {
        while let (Some(a), Some(b)) = (self.peek(self.current), self.peek(self.current + 1)) {
            match (a, b) {
                ('/', '/') => {
                    while let Some(c) = self.peek(self.current) {
                        if c != '\n' && !self.is_at_end() {
                            self.current += 1;
                        } else {
                            self.current += 1;
                            break;
                        }
                    }
                }
                (' ', _) | ('\r', _) | ('\t', _) => self.current += 1,
                ('\n', _) => {
                    self.line += 1;
                    self.current += 2;
                }
                _ => break,
            }
        }

        self.source.drain(self.start..self.current);
        self.start = 0;
        self.current = 0;
    }

    fn peek(&self, index: usize) -> Option<char> {
        self.source.chars().nth(index)
    }

    fn is_at_end(&self) -> bool {
        self.source.chars().nth(self.current).is_none()
    }

    fn make_token(&mut self, kind: TokenKind) -> Token {
        let token = Token::new(kind, String::from_iter(self.source.drain(self.start..self.current)), self.line);

        self.start = 0;
        self.current = 0;

        token
    }

    fn make_error_token(&self, message: &str) -> Token {
        Token::new(TokenKind::ERROR, message.to_string(), self.line)
    }
}
