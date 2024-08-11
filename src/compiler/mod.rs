mod parser;
mod helpers;
mod precedence;
mod rules;

use crate::chunk::Chunk;
use crate::compiler::parser::Parser;
use crate::scanner::token::TokenKind::EOF;

pub fn compile(chunk: &mut Chunk, source: &str) -> bool {
    let mut parser = Parser::new(chunk, source);
    parser.advance();

    while !parser.match_token(EOF) {
        parser.emit_declaration();
    }

    parser.end_compiler();
    !parser.had_error
}
