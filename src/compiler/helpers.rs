use super::precedence::Precedence;
use super::rules::ParseFn;
use super::rules::ParseRule;
use crate::scanner::token::TokenKind;
use std::collections::HashMap;

// TODO: IMPLIMENT INDEX TRAIT AND PASS IT TP PARSER

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
