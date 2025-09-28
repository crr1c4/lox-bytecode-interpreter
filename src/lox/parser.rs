use std::collections::VecDeque;

use super::token;
use super::token::Token;
use derive_more::Debug;
use derive_more::Display;

#[derive(Clone, Default, Display, Debug)]
#[display("{_0}")]
pub enum Value {
    Bool(bool),
    #[display("Nil")]
    #[debug("Nil")]
    #[default]
    Nil,
    Number(f64),
    String(String),
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
#[derive(Debug)]
pub enum Expression {
    #[debug("({operator} {left:?} {right:?} )")]
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    #[debug("(grouping {expression:?})")]
    Grouping { expression: Box<Expression> },
    #[debug("{value}")]
    Literal { value: Value },
    #[debug("({operator} {right:?})")]
    Unary { right: Box<Expression>, operator: Token },
}

struct Parser {
    tokens: VecDeque<Token>,
    current: usize,
}

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
