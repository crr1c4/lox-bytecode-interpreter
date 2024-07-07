use std::error::Error;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub source: String,
    pub line: u32,
}

impl Token {
    pub fn new(kind: TokenKind, source: String, line: u32) -> Self {
        Token { kind, source, line }
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String,
    Number,

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
    ERROR,
    EOF,
}

#[derive(Debug)]
pub struct TokenError<'a> {
    token: &'a Token,
    message: String,
}

impl<'a> Error for TokenError<'a> {}

impl<'a> Display for TokenError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let location = match self.token.kind {
            TokenKind::EOF => "at end",
            TokenKind::ERROR => "",
            _ => &format!("at '{}'", self.token.source),
        };

        write!(f, "[line {}] Error {}: {}", self.token.line, location, self.message)
    }
}

impl<'a> TokenError<'a> {
    pub fn new(token: &'a Token, message: impl Into<String>) -> Self {
        Self { token, message: message.into() }
    }
}
