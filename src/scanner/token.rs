#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenType,
    pub source: &'a str,
    pub line: i32,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenType, source: &'a str, line: i32) -> Self {
        Token {
            kind,
            source,
            line,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
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
