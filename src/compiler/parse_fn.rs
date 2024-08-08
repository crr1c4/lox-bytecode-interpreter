use std::collections::HashMap;

use crate::scanner::token::TokenKind;

pub fn create_rules() -> HashMap<TokenKind, ParseRule> {
    HashMap::from([
        (TokenKind::LeftParen, ParseRule::new(Some(ParseFn::Grouping), None, Precedence::None)),
        (TokenKind::RightParen, ParseRule::new(None, None, Precedence::None)),
        (TokenKind::LeftBrace, ParseRule::new(None, None, Precedence::None)),
        (TokenKind::RightBrace, ParseRule::new(None, None, Precedence::None)),
        (TokenKind::Comma, ParseRule::new(None, None, Precedence::None)),
        (TokenKind::Dot, ParseRule::new(None, None, Precedence::None)),
        (TokenKind::Minus, ParseRule::new(Some(ParseFn::Unary), Some(ParseFn::Binary), Precedence::Term)),
        (TokenKind::Plus, ParseRule::new(None, Some(ParseFn::Binary), Precedence::Term)),
        (TokenKind::Semicolon, ParseRule::new(None, None, Precedence::None)),
        (TokenKind::Slash, ParseRule::new(None, Some(ParseFn::Binary), Precedence::Factor)),
        (TokenKind::Star, ParseRule::new(None, Some(ParseFn::Binary), Precedence::Factor)),
        (TokenKind::Bang, ParseRule::new(Some(ParseFn::Unary), None, Precedence::None)),
        (TokenKind::BangEqual, ParseRule::new(None, Some(ParseFn::Binary), Precedence::Equality)),
        (TokenKind::Equal, ParseRule::new(None, None, Precedence::None)),
        (TokenKind::EqualEqual, ParseRule::new(None, Some(ParseFn::Binary), Precedence::Equality)),
        (TokenKind::Greater, ParseRule::new(None, Some(ParseFn::Binary), Precedence::Comparison)),
        (TokenKind::GreaterEqual, ParseRule::new(None, Some(ParseFn::Binary), Precedence::Comparison)),
        (TokenKind::Less, ParseRule::new(None, Some(ParseFn::Binary), Precedence::Comparison)),
        (TokenKind::LessEqual, ParseRule::new(None, Some(ParseFn::Binary), Precedence::Comparison)),
        (TokenKind::Identifier, ParseRule::new(Some(ParseFn::Variable), None, Precedence::None)),
        (TokenKind::String, ParseRule::new(Some(ParseFn::String), None, Precedence::None)),
        (TokenKind::Number, ParseRule::new(Some(ParseFn::Number), None, Precedence::None)),
        (TokenKind::And, ParseRule::new(None, None, Precedence::None)),
        (TokenKind::Class, ParseRule::new(None, None, Precedence::None)),
        (TokenKind::Else, ParseRule::new(None, None, Precedence::None)),
        (TokenKind::False, ParseRule::new(Some(ParseFn::Literal), None, Precedence::None)),
        (TokenKind::For, ParseRule::new(None, None, Precedence::None)),
        (TokenKind::Fun, ParseRule::new(None, None, Precedence::None)),
        (TokenKind::If, ParseRule::new(None, None, Precedence::None)),
        (TokenKind::Nil, ParseRule::new(Some(ParseFn::Literal), None, Precedence::None)),
        (TokenKind::Or, ParseRule::new(None, None, Precedence::None)),
        (TokenKind::Print, ParseRule::new(None, None, Precedence::None)),
        (TokenKind::Return, ParseRule::new(None, None, Precedence::None)),
        (TokenKind::Super, ParseRule::new(None, None, Precedence::None)),
        (TokenKind::This, ParseRule::new(None, None, Precedence::None)),
        (TokenKind::True, ParseRule::new(Some(ParseFn::Literal), None, Precedence::None)),
        (TokenKind::Var, ParseRule::new(None, None, Precedence::None)),
        (TokenKind::While, ParseRule::new(None, None, Precedence::None)),
        (TokenKind::ERROR, ParseRule::new(None, None, Precedence::None)),
        (TokenKind::EOF, ParseRule::new(None, None, Precedence::None)),
    ])
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None = 0,
    Assignment = 1,
    Or = 2,
    And = 3,
    Equality = 4,
    Comparison = 5,
    Term = 6,
    Factor = 7,
    Unary = 8,
    Call = 9,
    Primary = 10,
}

/* impl From<u32> for Precedence {
    fn from(value: u32) -> Self {
        match value {
            0 => Precedence::None,
            1 => Precedence::Assignment,
            2 => Precedence::Or,
            3 => Precedence::And,
            4 => Precedence::Equality,
            5 => Precedence::Comparison,
            6 => Precedence::Term,
            7 => Precedence::Factor,
            8 => Precedence::Unary,
            9 => Precedence::Call,
            10 => Precedence::Primary,
            // WARNING: Check if its true xd.
            _ => unreachable!(),
        }
    }
} */

#[derive(Debug)]
pub enum ParseFn {
    Grouping,
    Unary,
    Binary,
    Number,
    Literal,
    String,
    Variable,
}

#[derive(Debug)]
pub struct ParseRule {
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: Precedence,
}

impl ParseRule {
    pub fn new(prefix: Option<ParseFn>, infix: Option<ParseFn>, precedence: Precedence) -> Self {
        Self { prefix, infix, precedence }
    }
}
