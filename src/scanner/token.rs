#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub source: String,
    pub line: i32,
}

impl Token {
    pub fn new(kind: TokenKind, source: String, line: i32) -> Self {
        Token { kind, source, line }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
