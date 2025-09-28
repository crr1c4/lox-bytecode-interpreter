use crate::Line;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: Kind,
    pub lexeme: Option<String>,
    pub line: Line
}

impl Token {
    pub fn new(kind: Kind, line: Line, lexeme: Option<String>) -> Self {
        Token { kind, line, lexeme }
    }
}


#[derive(Debug, PartialEq)]
pub enum Kind {
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
    // ERROR,
    EOF,
}

