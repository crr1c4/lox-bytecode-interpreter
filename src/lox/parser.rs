// use std::collections::VecDeque;
use super::chunk::Chunk;
use crate::error::{LexicalError, SyntaxError};
use crate::lox::Line;
use crate::lox::opcode::OpCode;
use crate::lox::scanner::Scanner;
use crate::lox::value::Value;
use std::iter::Peekable;

use super::token;

/*
 * MAYBE PUT COMPILE FUNTION UN LOX.RS? I THINK IT'S THE BEST LOCATION FOR THAT.
 *
 */

struct Parser {
    scanner: Peekable<Scanner>,
    pub chunk: Chunk,
}

impl Parser {
    pub fn new(input: String) -> Self {
        Self {
            scanner: Scanner::new(input).peekable(),
            chunk: Chunk::new(),
        }
    }

    fn parse_expression(&mut self) {
        // Prefix
        // let lhs =
    }

    // Prefix expressions.
    fn number(&mut self) -> Result<(), LexicalError> {
        match self.scanner.next() {
            Some(Ok(token)) if token.kind.eq(&token::Kind::Number) => {
                let value = token.lexeme.unwrap().parse::<f64>().unwrap();
                let line = token.line;

                self.emit(OpCode::Constant(Value::Number(value)), line);
            }
            Some(Err(lexical_error)) => {
                return Err(lexical_error);
            }
            _ => (),
        };

        Ok(())
    }

    /// Checks for an expected token kind.
    pub fn consume<F>(&mut self, kind: token::Kind, make_syntax_error: F) -> Result<(), SyntaxError>
    where
        F: FnOnce(Line) -> SyntaxError,
    {
        if let Some(Ok(token)) = self.scanner.next()
            && token.kind.ne(&kind)
        {
            // syntax_error.0 = token.line;
            return Err(make_syntax_error(token.line));
        }

        Ok(())
    }

    fn grouping(&mut self) -> Result<(), SyntaxError> {
        self.expression();
        self.consume(token::Kind::RightParen, |line| SyntaxError::ExpectedRightParen(line))?;
        Ok(())
    }

    fn expression(&mut self) {}

    fn emit(&mut self, opcode: OpCode, line: Line) {
        self.chunk.write(opcode, line);
    }
}
// struct Parser {
//     tokens: VecDeque<Token>,
//     current: usize,
// }

// impl Parser {
//     pub fn new(tokens: VecDeque<Token>) -> Self {
//         Self { tokens, current: 0 }
//     }
//
//     /// expression -> equality
//     fn expression(&mut self) -> Expression {
//         self.equality()
//     }
//
//     fn equality(&mut self) -> Expression {
//         let left = self.comparison();
//
//         let Some(token) = self.tokens.front() else {
//
//         }
//             // return error;
//
//             // if token.kind.eq(token::Kind::BangEqual) || self.match_token(token::Kind::EqualEqual) {}
//             //
//             // let operator = self.tokens.rem(self.current - 1).unwrap();
//             // let right = self.comparison();
//             // Expression::Binary {
//             //     left: Box::new(left),
//             //     operator,
//             //     right: Box::new(right),
//         // };
//
//         Expression::Binary { left: (), operator: (), right: () }
//     }
//
//     fn r#match(&mut self, expected: &[token::Kind]) -> bool {
//         if let Some(Token { kind, .. }) = self.tokens.get(self.current) {
//             if expected.con(kind) {
//                 self.current += 1;
//                 return true;
//             }
//         }
//
//         false
//     }
//
//     fn comparison(&mut self) -> Expression {}
// }
//
//
// //
// // // use crate::lox::chunk::Chunk;
// // // use vm::VirtualMachine;
// //
// // pub fn compile(chunk: &mut Chunk, source: &str) -> Result<()> {
// //     // let mut parser = Parser::new(chunk, source);
// //     // parser.advance();
// //     //
// //     // while !parser.match_token(EOF) {
// //     //     parser.emit_declaration();
// //     // }
// //     //
// //     // parser.end_compiler();
// //     // // !parser.had_error
// //     Ok(())
// // }
// //
// // pub fn interpret(source: &str, debug: bool, vm: &mut VirtualMachine) -> Result<()> {
// //     let mut chunk = Chunk::new();
// //     compile(&mut chunk, source)?;
// //
// //     if debug {
// //         println!("{:?}", chunk);
// //     }
// //
// //     vm.run(chunk)?;
// //
// //     Ok(())
// // }
