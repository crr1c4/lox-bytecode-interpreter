// TODO: Check unwrap() lines.
// BUG: Primitive expressions cause loops.

use crate::chunk::op_code::OpCode;
use crate::chunk::Chunk;
use crate::scanner::token::*;
use crate::scanner::Scanner;
use crate::value::Value;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    pub fn emit(&mut self, code: OpCode) {
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
                Some(ParseFn::Literal) => self.literal(),
                Some(ParseFn::String) => self.string(),
                Some(ParseFn::Variable) => self.variable(),
                None => {
                    self.error_at(token.clone(), "Expect expression.");
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
                Some(ParseFn::Literal) => self.literal(),
                Some(ParseFn::String) => self.string(),
                Some(ParseFn::Variable) => self.variable(),
                None => (),
            }
        }
    }

    fn number(&mut self) {
        if let Some(token) = &self.previous_token {
            let value = token.source.parse::<f64>().unwrap();
            let code = OpCode::Constant(Value::Number(value));
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

            match operator_kind {
                TokenKind::Minus => self.emit(OpCode::Negate),
                TokenKind::Bang => self.emit(OpCode::Not),
                _ => unreachable!(),
            }
        }
    }

    fn binary(&mut self) {
        if let Some(token) = &self.previous_token {
            let operator_kind = token.kind;
            let rule = self.rules.get(&operator_kind).unwrap();
            self.parse_precedence(Precedence::from(rule.precedence as u32 + 1));

            match operator_kind {
                TokenKind::Plus => self.emit(OpCode::Add),
                TokenKind::Minus => self.emit(OpCode::Substract),
                TokenKind::Star => self.emit(OpCode::Multiply),
                TokenKind::Slash => self.emit(OpCode::Divide),

                TokenKind::EqualEqual => self.emit(OpCode::Equal),
                TokenKind::Greater => self.emit(OpCode::Greater),
                TokenKind::Less => self.emit(OpCode::Less),
                TokenKind::BangEqual => {
                    self.emit(OpCode::Equal);
                    self.emit(OpCode::Not);
                }
                TokenKind::LessEqual => {
                    self.emit(OpCode::Greater);
                    self.emit(OpCode::Not);
                }
                TokenKind::GreaterEqual => {
                    self.emit(OpCode::Less);
                    self.emit(OpCode::Not);
                }
                _ => unreachable!(),
            }
        }
    }

    fn literal(&mut self) {
        if let Some(token) = &self.previous_token {
            match token.kind {
                TokenKind::Nil => self.emit(OpCode::Nil),
                TokenKind::False => self.emit(OpCode::False),
                TokenKind::True => self.emit(OpCode::True),
                _ => unreachable!(),
            }
        }
    }

    fn string(&mut self) {
        if let Some(token) = &self.previous_token {
            let value = token.source.get(1..token.source.len() - 1);
            let value = String::from_iter(value);
            let value = Value::from(value);
            self.emit(OpCode::Constant(value));
        }
    }

    fn variable(&mut self) {
        self.named_variable();
    }

    fn named_variable(&mut self) {
        let arg = self.identifier_constant();

        if self.match_token(TokenKind::Equal) {
            self.expression();
            self.emit(OpCode::SetGlobal);
            self.emit(arg);
        } else {
            self.emit(OpCode::GetGlobal);
            self.emit(arg);
        }
    }

    // ==================== Expressions and statements. ===================================0

    pub fn match_token(&mut self, kind: TokenKind) -> bool {
        if !self.check(kind) {
            return false;
        }

        self.advance();
        true
    }

    fn check(&self, kind: TokenKind) -> bool {
        if let Some(token) = &self.current_token {
            return token.kind == kind;
        }

        false
    }

    pub fn declaration(&mut self) {
        if self.match_token(TokenKind::Var) {
            self.var_declaration();
        } else {
            self.statement();
        }

        if self.panic_mode {
            self.synchronize();
        }
    }

    fn var_declaration(&mut self) {
        let global = self.parse_variable("Expect variable name.");

        if self.match_token(TokenKind::Equal) {
            self.expression();
        } else {
            self.emit(OpCode::Nil);
        }

        self.consume(TokenKind::Semicolon, "Expect ',' after variable declaration.");
        self.define_variable(global);
    }

    fn define_variable(&mut self, global: OpCode) {
        self.emit(OpCode::DefineGlobal);
        self.emit(global);
    }

    fn parse_variable(&mut self, message: &str) -> OpCode {
        self.consume(TokenKind::Identifier, message);
        self.identifier_constant()
    }

    fn identifier_constant(&self) -> OpCode {
        if let Some(token) = &self.previous_token {
            OpCode::Constant(Value::Object(Box::new(token.source.clone())))
        } else {
            unreachable!()
        }
    }

    fn synchronize(&mut self) {
        self.panic_mode = false;

        while let Some(token) = &self.current_token {
            if token.kind == TokenKind::EOF {
                return;
            }

            if let Some(previous) = &self.previous_token {
                if previous.kind == TokenKind::Semicolon {
                    return;
                }
            }

            match token.kind {
                TokenKind::Class | TokenKind::Fun | TokenKind::Var | TokenKind::For | TokenKind::If | TokenKind::While | TokenKind::Print | TokenKind::Return => return,
                _ => (),
            }

            self.advance();
        }
    }

    fn statement(&mut self) {
        if self.match_token(TokenKind::Print) {
            self.print_statement();
        } else {
            self.expression_statement();
        }
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.consume(TokenKind::Semicolon, "Expect ';' after expression");
        self.emit(OpCode::Pop);
    }

    fn print_statement(&mut self) {
        self.expression();
        self.consume(TokenKind::Semicolon, "Expect ';' after value.");
        self.emit(OpCode::Print)
    }
}
