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
