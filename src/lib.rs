pub mod chunk;
pub mod opcode;
pub mod value;
// mod cli;
// mod compiler;
mod error;
mod scanner;
mod vm;

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
