use crate::Line;
use derive_more::Debug;
use derive_more::Display;

#[derive(Debug, Display, PartialEq)]
#[display("{}", match self.kind {
    Kind::Identifier | Kind::Number | Kind::String => self.lexeme.as_deref().unwrap_or("").to_string(),
    _ => self.kind.to_string()
})]
pub struct Token {
    pub kind: Kind,
    pub lexeme: Option<String>,
    pub line: Line,
}

impl Token {
    pub fn new(kind: Kind, line: Line, lexeme: Option<String>) -> Self {
        Token { kind, line, lexeme }
    }
}

#[derive(Debug, Display, PartialEq)]
pub enum Kind {
    #[display("(")]
    LeftParen,
    #[display(")")]
    RightParen,
    #[display("{{")]
    LeftBrace,
    #[display("}}")]
    RightBrace,
    #[display(",")]
    Comma,
    #[display(".")]
    Dot,
    #[display("-")]
    Minus,
    #[display("+")]
    Plus,
    #[display(";")]
    Semicolon,
    #[display("/")]
    Slash,
    #[display("*")]
    Star,

    #[display("!")]
    Bang,
    #[display("!=")]
    BangEqual,
    #[display("=")]
    Equal,
    #[display("==")]
    EqualEqual,
    #[display(">")]
    Greater,
    #[display(">=")]
    GreaterEqual,
    #[display("<")]
    Less,
    #[display("<=")]
    LessEqual,

    #[display("")]
    Identifier,
    #[display("")]
    String,
    #[display("")]
    Number,

    #[display("and")]
    And,
    #[display("class")]
    Class,
    #[display("else")]
    Else,
    #[display("false")]
    False,
    #[display("fun")]
    Fun,
    #[display("for")]
    For,
    #[display("if")]
    If,
    #[display("nil")]
    Nil,
    #[display("or")]
    Or,
    #[display("print")]
    Print,
    #[display("return")]
    Return,
    #[display("super")]
    Super,
    #[display("false")]
    This,
    #[display("true")]
    True,
    #[display("var")]
    Var,
    #[display("while")]
    While,
    // #[debug("(")]
    // ERROR,
    #[display("EOF")]
    EOF,
}
