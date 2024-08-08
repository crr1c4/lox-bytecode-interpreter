mod parser;
mod parse_fn;

use crate::chunk::op_code::OpCode;
use crate::chunk::Chunk;
use crate::compiler::parser::Parser;
use crate::scanner::token::TokenKind;

pub fn compile(chunk: &mut Chunk, source: &str) -> bool {
    let mut parser = Parser::new(chunk, source);
    parser.advance();

    while !parser.match_token(TokenKind::EOF) {
        parser.declaration();
    }
    // parser.expression();
    // parser.consume(TokenKind::EOF, "Expect end of expression");
    parser.emit(OpCode::Return);
    !parser.had_error
}

/* pub fn compile(source: &str) {
    let mut scanner = crate::scanner::Scanner::new(source);
    let mut line = 0;

    while let Some(token) = scanner.next() {
        if token.line != line {
            print!("{:>4} ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }

        println!("{:>2} '{}'", token.kind as usize, token.source);
    }
} */
