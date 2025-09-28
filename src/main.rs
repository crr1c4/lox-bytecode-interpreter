mod lox;
mod cli;
mod error;

use anyhow::Result;

type Line = u32;
type Identifier = String;

use crate::chunk::Chunk;
use vm::VirtualMachine;
// use crate::compiler::parser::Parser;
// use crate::scanner::token::TokenKind::EOF;

pub fn compile(chunk: &mut Chunk, source: &str) -> Result<()> {
    // let mut parser = Parser::new(chunk, source);
    // parser.advance();
    //
    // while !parser.match_token(EOF) {
    //     parser.emit_declaration();
    // }
    //
    // parser.end_compiler();
    // // !parser.had_error
    Ok(())
}

pub fn interpret(source: &str, debug: bool, vm: &mut VirtualMachine) -> Result<()> {
    let mut chunk = Chunk::new();
    compile(&mut chunk, source)?;

    if debug {
        println!("{:?}", chunk);
    }

    vm.run(chunk)?;

    Ok(())
}
// TODO: Write tests.
// TODO: Refactor code.
// TODO: Add thiserror crate.
// TODO: Add docs.

use lox::chunk::Chunk;
use lox::opcode::OpCode;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::Add, 1);
    chunk.write(OpCode::Add, 2);
    chunk.write(OpCode::Constant(4.56.into()), 3);
    chunk.write(OpCode::Constant("hola".to_string().into()), 3);
    println!("{:?}", chunk);
}
