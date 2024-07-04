// WARNING: Check unwrap() lines.
// BUG: Check debug chunk line bug.

use crate::chunk::op_code::OperationCode;
use crate::chunk::Chunk;
use crate::scanner::token::*;
use crate::scanner::Scanner;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl From<u32> for Precedence {
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
}

#[derive(Debug)]
enum ParseFn {
    Grouping,
    Unary,
    Binary,
    Number,
}

#[derive(Debug)]
pub struct ParseRule {
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: Precedence,
}

impl ParseRule {
    fn new(prefix: Option<ParseFn>, infix: Option<ParseFn>, precedence: Precedence) -> Self {
        Self { prefix, infix, precedence }
    }
}

#[derive(Debug)]
pub struct Parser<'a> {
    current_token: Option<Box<Token>>,
    previous_token: Option<Box<Token>>,
    scanner: Scanner,
    compiling_chunk: &'a mut Chunk,
    pub had_error: bool,
    panic_mode: bool,
    rules: HashMap<TokenKind, ParseRule>,
}

impl<'a> Parser<'a> {
    pub fn new(chunk: &'a mut Chunk, source: &str) -> Self {
        let rules = HashMap::from([
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
            (TokenKind::Bang, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::BangEqual, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::Equal, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::EqualEqual, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::Greater, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::GreaterEqual, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::Less, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::LessEqual, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::Identifier, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::String, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::Number, ParseRule::new(Some(ParseFn::Number), None, Precedence::None)),
            (TokenKind::And, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::Class, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::Else, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::False, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::For, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::Fun, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::If, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::Nil, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::Or, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::Print, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::Return, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::Super, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::This, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::True, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::Var, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::While, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::ERROR, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::EOF, ParseRule::new(None, None, Precedence::None)),
        ]);

        Self {
            compiling_chunk: chunk,
            scanner: Scanner::new(source),
            current_token: None,
            previous_token: None,
            had_error: false,
            panic_mode: false,
            rules,
        }
    }

    pub fn advance(&mut self) {
        self.previous_token = self.current_token.clone();

        // NOTE: The loop is needed for "ignoring" possbile error tokens.
        while let Some(token) = self.scanner.next() {
            self.current_token = Some(Box::new(token));

            match &self.current_token {
                Some(token) if token.kind == TokenKind::ERROR => self.error_at(token.clone(), ""),
                _ => break,
            }
        }
    }

    pub fn consume(&mut self, expected_kind: TokenKind, message: &str) {
        if let Some(token) = &self.current_token {
            if token.kind == expected_kind {
                self.advance();
            } else {
                self.error_at(token.clone(), message);
            }
        }
    }

    fn error_at(&mut self, token: Box<Token>, message: &str) {
        if self.panic_mode {
            return;
        }

        eprintln!("{}", TokenError::new(&token, message));
        self.panic_mode = true;
        self.had_error = true;
    }

    pub fn emit(&mut self, code: OperationCode) {
        if let Some(token) = &self.current_token {
            self.compiling_chunk.write(code, token.line);
        }
    }

    pub fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        if let Some(token) = &self.previous_token {
            let rule = self.rules.get(&token.kind).unwrap();
            match rule.prefix {
                Some(ParseFn::Unary) => self.unary(),
                Some(ParseFn::Binary) => self.binary(),
                Some(ParseFn::Number) => self.number(),
                Some(ParseFn::Grouping) => self.grouping(),
                None => {
                    self.error_at(token.clone(), "Expect expression");
                    return;
                }
            }
        }

        while let Some(token) = &self.current_token.clone() {
            if precedence as u32 > self.rules.get(&token.kind).unwrap().precedence as u32 {
                break;
            }

            self.advance();

            match self.rules.get(&token.kind).unwrap().infix {
                Some(ParseFn::Unary) => self.unary(),
                Some(ParseFn::Binary) => self.binary(),
                Some(ParseFn::Number) => self.number(),
                Some(ParseFn::Grouping) => self.grouping(),
                None => (),
            }
        }
    }

    fn number(&mut self) {
        if let Some(token) = &self.previous_token {
            let value = token.source.parse::<f64>().unwrap();
            let code = OperationCode::Constant(value);
            self.emit(code);
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenKind::RightParen, "Expect ')' after expression");
    }

    fn unary(&mut self) {
        if let Some(token) = &self.previous_token {
            let operator_kind = token.kind;
            self.parse_precedence(Precedence::Unary);

            if operator_kind == TokenKind::Minus {
                self.emit(OperationCode::Negate);
            }
        }
    }

    fn binary(&mut self) {
        if let Some(token) = &self.previous_token {
            let operator_kind = token.kind;
            let rule = self.rules.get(&operator_kind).unwrap();
            self.parse_precedence(Precedence::from(rule.precedence as u32 + 1));

            match operator_kind {
                TokenKind::Plus => self.emit(OperationCode::Add),
                TokenKind::Minus => self.emit(OperationCode::Substract),
                TokenKind::Star => self.emit(OperationCode::Multiply),
                TokenKind::Slash => self.emit(OperationCode::Divide),
                _ => unreachable!(),
            }
        }
    }
}
