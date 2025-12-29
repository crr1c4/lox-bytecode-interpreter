use std::iter::Scan;

use crate::lox::{chunk::Chunk, scanner::Scanner, token::Token};
use anyhow::Result;

/// Interprets a chunk of source code.
pub fn interpret(source: String) -> Result<()> {
    // TODO: RUN COMPILED CHUNK ON VM
    match compile(source) {
        Ok(chunk) => todo!(), /* vm::run(chunk) */
        Err(err) => todo!(),  /* report error */
    }
}

fn compile(source: String) -> Result<Chunk> {
    Parser::init();
}

struct Parser {
    previous_token: Option<Token>,
    current_token: Option<Token>,
    scanner: Scanner,
    chunk
}

impl Parser {
    fn init(source: String) -> Self {
        Self {
            scanner: Scanner::new(source),
            current_token: None,
            previous_token: None,
        }
    }
}
