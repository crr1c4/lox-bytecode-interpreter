// TODO: Check unwrap() lines.

use super::helpers::create_rules;
use super::precedence::Precedence;
use super::rules::ParseFn;
use super::rules::ParseRule;
use crate::chunk::opcode::OpCode;
use crate::chunk::Chunk;
use crate::scanner::token::*;
use crate::scanner::Scanner;
use crate::value::Value;
use std::collections::HashMap;

// NOTE: Local variables structs.
// TODO: Move to compiler/mod.rs

#[derive(Default, Debug)]
struct Compiler {
    locals: Vec<Local>,
    scope_depth: u32,
}

// if self.current_compiler.count == u8::MAX.into() + 1 {
//     self.error_at(Box::new(name), "Too many local variables in function.");
//     return;
// }
//
// if let Some(local) = self.current_compiler.locals.get(self.current_compiler.count) {
//    k

impl Compiler {
    fn add_local(&mut self, name: &Token) {
        self.locals.push(Local {
            name: name.clone(),
            depth: self.scope_depth,
        });
    }

    // fn depth(&self) -> u32 {
    //     self.locals.iter()
    // }

    fn count(&self) -> u32 {
        self.locals.len() as u32
    }
}

#[derive(Debug)]
struct Local {
    name: Token,
    depth: u32,
}

impl Compiler {
    fn initialize() -> Self {
        Self::default()
    }
}

// ------------------------------

#[derive(Debug)]
pub struct Parser<'a> {
    current_token: Option<Box<Token>>,
    previous_token: Option<Box<Token>>,
    scanner: Scanner,
    compiling_chunk: &'a mut Chunk,
    pub had_error: bool,
    panic_mode: bool,
    rules: HashMap<TokenKind, ParseRule>,
    current_compiler: Compiler,
}

impl<'a> Parser<'a> {
    pub fn new(chunk: &'a mut Chunk, source: &str) -> Self {
        Self {
            compiling_chunk: chunk,
            scanner: Scanner::new(source),
            current_token: None,
            previous_token: None,
            had_error: false,
            panic_mode: false,
            rules: create_rules(),
            current_compiler: Compiler::initialize(),
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

    pub fn emit_byte(&mut self, code: OpCode) {
        if let Some(token) = &self.current_token {
            self.compiling_chunk.write(code, token.line);
        }
    }

    pub fn emit_bytes(&mut self, code1: OpCode, code2: OpCode) {
        if let Some(token) = &self.current_token {
            self.compiling_chunk.write(code1, token.line);
            self.compiling_chunk.write(code2, token.line);
        }
    }

    pub fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return);
    }

    // NOTE: Expressions.

    pub fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let can_assign = precedence <= Precedence::Assignment;

        if let Some(token) = &self.previous_token {
            let rule = self.rules.get(&token.kind).unwrap();

            match rule.prefix {
                Some(ParseFn::Unary) => self.emit_unary(can_assign),
                Some(ParseFn::Binary) => self.emit_binary(can_assign),
                Some(ParseFn::Number) => self.emit_number(can_assign),
                Some(ParseFn::Grouping) => self.emit_grouping(can_assign),
                Some(ParseFn::Literal) => self.emit_literal(can_assign),
                Some(ParseFn::String) => self.emit_string(can_assign),
                Some(ParseFn::Variable) => self.emit_variable(can_assign),
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
                Some(ParseFn::Unary) => self.emit_unary(can_assign),
                Some(ParseFn::Binary) => self.emit_binary(can_assign),
                Some(ParseFn::Number) => self.emit_number(can_assign),
                Some(ParseFn::Grouping) => self.emit_grouping(can_assign),
                Some(ParseFn::Literal) => self.emit_literal(can_assign),
                Some(ParseFn::String) => self.emit_string(can_assign),
                Some(ParseFn::Variable) => self.emit_variable(can_assign),
                None => (),
            }
        }

        let match_equal = self.match_token(TokenKind::Equal);
        if let Some(token) = &self.previous_token {
            if can_assign && match_equal {
                self.error_at(token.clone(), "Invalid assignment target.");
            }
        }
    }

    fn emit_number(&mut self, _can_assign: bool) {
        if let Some(token) = &self.previous_token {
            let value = token.source.parse::<f64>().unwrap();
            let code = OpCode::Constant(Value::Number(value));
            self.emit_byte(code);
        }
    }

    fn emit_grouping(&mut self, _can_assign: bool) {
        self.expression();
        self.consume(TokenKind::RightParen, "Expect ')' after expression");
    }

    fn emit_unary(&mut self, _can_assign: bool) {
        if let Some(token) = &self.previous_token {
            let operator_kind = token.kind;
            self.parse_precedence(Precedence::Unary);

            match operator_kind {
                TokenKind::Minus => self.emit_byte(OpCode::Negate),
                TokenKind::Bang => self.emit_byte(OpCode::Not),
                _ => unreachable!(),
            }
        }
    }

    fn emit_binary(&mut self, _can_assign: bool) {
        if let Some(token) = &self.previous_token {
            let operator_kind = token.kind;
            let rule = self.rules.get(&operator_kind).unwrap();

            if let Ok(precedence) = Precedence::try_from(u8::from(rule.precedence) + 1) {
                self.parse_precedence(precedence);
            }

            // TODO: add err message for precedence.

            match operator_kind {
                TokenKind::Plus => self.emit_byte(OpCode::Add),
                TokenKind::Minus => self.emit_byte(OpCode::Substract),
                TokenKind::Star => self.emit_byte(OpCode::Multiply),
                TokenKind::Slash => self.emit_byte(OpCode::Divide),

                TokenKind::EqualEqual => self.emit_byte(OpCode::Equal),
                TokenKind::Greater => self.emit_byte(OpCode::Greater),
                TokenKind::Less => self.emit_byte(OpCode::Less),
                TokenKind::BangEqual => {
                    self.emit_byte(OpCode::Equal);
                    self.emit_byte(OpCode::Not);
                }
                TokenKind::LessEqual => {
                    self.emit_byte(OpCode::Greater);
                    self.emit_byte(OpCode::Not);
                }
                TokenKind::GreaterEqual => {
                    self.emit_byte(OpCode::Less);
                    self.emit_byte(OpCode::Not);
                }
                _ => unreachable!(),
            }
        }
    }

    fn emit_literal(&mut self, _can_assign: bool) {
        if let Some(token) = &self.previous_token {
            match token.kind {
                TokenKind::Nil => self.emit_byte(OpCode::Nil),
                TokenKind::False => self.emit_byte(OpCode::False),
                TokenKind::True => self.emit_byte(OpCode::True),
                _ => unreachable!(),
            }
        }
    }

    fn emit_string(&mut self, _can_assign: bool) {
        if let Some(token) = &self.previous_token {
            let value = token.source.get(1..token.source.len() - 1);
            let value = String::from_iter(value);
            let value = Value::from(value);
            self.emit_byte(OpCode::Constant(value));
        }
    }

    // NOTE: Expressions and statements.

    /// If the current token has the given kind, we consume the token and return true.
    /// Otherwise, we leave the token alone and return false.
    pub fn match_token(&mut self, kind: TokenKind) -> bool {
        let token_match = self.check(kind);

        if token_match {
            self.advance();
        }

        token_match
    }

    /// Checks if the token has the given kind.
    fn check(&self, kind: TokenKind) -> bool {
        match &self.current_token {
            Some(token) => token.kind == kind,
            _ => false,
        }
    }

    /// The declaration rule contains the statements that declare names, and also includes
    /// statements so that all statement type are allowed.
    ///
    /// If we hit a compile error while parsing the previous statement, we enter panic mode. When
    /// that happens, wr start synchronizing.
    pub fn emit_declaration(&mut self) {
        if self.match_token(TokenKind::Var) {
            self.emit_var_declaration();
        } else {
            self.emit_statement();
        }

        if self.panic_mode {
            self.synchronize();
        }
    }

    /// The method skips tokens until we reach something that looks like a statement boundary. We recognize
    /// the boundary. The method recognize the boundary by looking for a preceding token that can
    /// end an statement, like a semicolon. Or we'll look for a subsequent token that begin and
    /// statement, usually one of the control flow or declaration keywords.
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
                TokenKind::Class
                | TokenKind::Fun
                | TokenKind::Var
                | TokenKind::For
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Print
                | TokenKind::Return => return,
                _ => (),
            }

            self.advance();
        }
    }

    /// TODO: complete docs here
    /// If we don't see a print keyword, the we must be looking at an expression statement.
    fn emit_statement(&mut self) {
        if self.match_token(TokenKind::Print) {
            self.emit_print_statement();
        } else if self.match_token(TokenKind::LeftBrace) {
            self.begin_scope();
            self.emit_block();
            self.end_scope();
        } else {
            self.emit_expression_statement();
        }
    }

    /// An expression statement is simply an expression followed by a semicolon.
    /// Semantically, an expression statement evaluates the expression and discards the result,
    /// and emit and OP_POP instruction.
    fn emit_expression_statement(&mut self) {
        self.expression();
        self.consume(TokenKind::Semicolon, "Expect ';' after expression");
        self.emit_byte(OpCode::Pop);
    }

    /// A print statement evaluates an expression and prints the result,
    /// so we first parse and compile the expression. The grammar expects a semicolon after that.
    /// Finally, we emit a new instruction to print the result.
    fn emit_print_statement(&mut self) {
        self.expression();
        self.consume(TokenKind::Semicolon, "Expect ';' after value.");
        self.emit_byte(OpCode::Print)
    }

    /// NOTE: Global Variables methods.

    /// First, the method compiles the variable name through parse_variable(). Then we look for an
    /// = followed by a initializer expression. If the user doesn't initialize the variable. the
    /// compiler implicity initializes it to nil. Either way, we expect the statement to be
    /// terminated with a semicolon.
    fn emit_var_declaration(&mut self) {
        let global = self.parse_variable("Expect variable name.");

        if self.match_token(TokenKind::Equal) {
            self.expression();
        } else {
            self.emit_byte(OpCode::Nil);
        }

        self.consume(TokenKind::Semicolon, "Expect ',' after variable declaration.");
        self.define_variable(global);
    }

    fn parse_variable(&mut self, message: &str) -> OpCode {
        self.consume(TokenKind::Identifier, message);

        self.declare_variable();
        if self.current_compiler.scope_depth > 0 {
            // TODO: Check this
            // BUG: Parse this option opcode return value. it interprets a return , manage it with
            // if let :)
            return OpCode::Return;
        }

        self.parse_identifier_constant()
    }

    /// Creates a constant OpCode that contains the variable name.
    fn parse_identifier_constant(&self) -> OpCode {
        if let Some(token) = &self.previous_token {
            OpCode::Constant(Value::Object(Box::new(token.source.clone())))
        } else {
            unreachable!()
        }
    }

    fn emit_variable(&mut self, can_assign: bool) {
        self.emit_named_variable(can_assign);
    }

    /// Toke the given identifier token and add its lexeme to the chunk's constant table as a
    /// string.
    ///
    /// Since assignment is the lowest precedence expression, the only time we allow an assignment
    /// is when parsing an assignment expression or top level expression like in an expression
    /// statement.
    fn emit_named_variable(&mut self, can_assign: bool) {
        let get_op: OpCode;
        let set_op: OpCode;


        // let arg = self.resolve_constant();

        let arg = self.parse_identifier_constant();

        if self.match_token(TokenKind::Equal) && can_assign {
            self.expression();
            self.emit_byte(arg);
            self.emit_byte(OpCode::SetGlobal);
        } else {
            self.emit_byte(arg);
            self.emit_byte(OpCode::GetGlobal);
        }
    }

    fn define_variable(&mut self, global: OpCode) {
        if self.current_compiler.scope_depth > 0 {
            return;
        }

        self.emit_byte(global);
        self.emit_byte(OpCode::DefineGlobal);
    }

    fn declare_variable(&mut self) {
        // If we're in the top-level global scope, we just bail out.
        if self.current_compiler.scope_depth == 0 {
            return;
        }

        // For local variables it needs to remember that the variable exists.
        if let Some(name) = &self.previous_token {
            // Shadowing verification.
            for local in self.current_compiler.locals.iter().rev() {
                if local.depth != 1 && local.depth < self.current_compiler.scope_depth {
                    break;
                }

                // WARNING: CHANGE REFERENCES AND USE THIS ERROR LABRERY
                //
                if *name.as_ref() == local.name {
                    // self.error_at(Box::new(local.name.clone()), "Already a variable with this name in the scope.")
                    eprintln!("Already a variable with this name in the scope.")
                }
            }

            self.current_compiler.add_local(name);
        }
    }

    // NOTE: Block statements.

    /// It keeps parsing declarations and statements until it hits the closing brace. As we do with
    /// any loop in the parser, we also checl for the end of the token stream.
    fn emit_block(&mut self) {
        while !self.check(TokenKind::RightBrace) && !self.check(TokenKind::EOF) {
            self.emit_declaration();
        }

        self.consume(TokenKind::RightBrace, "Expect '}' after block.");
    }

    fn begin_scope(&mut self) {
        self.current_compiler.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.current_compiler.scope_depth -= 1;

        // WARNING: check number
        // TODO: CHECK FUNCTIONAL METHOD
        let mut pops = 0;

        for local in self.current_compiler.locals.iter().rev() {
            if self.current_compiler.count() > 0 && local.depth > self.current_compiler.scope_depth {
                pops += 1;
            }
        }

        self.current_compiler.scope_depth -= pops;

        for _ in 1..=pops {
             self.emit_byte(OpCode::Pop);
        }


    }
}
