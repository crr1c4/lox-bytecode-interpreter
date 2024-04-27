pub mod chunk;
pub mod debug;
pub mod value;
pub mod vm;

use crate::chunk::*;
use crate::debug::disassemble_chunk;
use crate::vm::VirtualMachine;

fn main() {
    let mut chunk = Chunk::create();
    chunk.write(OperationCode::Constant(1.2), 123);
    chunk.write(OperationCode::Constant(3.14), 123);

    chunk.write(OperationCode::Add, 123);
    chunk.write(OperationCode::Constant(5.6), 123);
    chunk.write(OperationCode::Divide, 123);
    chunk.write(OperationCode::Negate, 123);

    chunk.write(OperationCode::Return, 123);
    // println!("{chunk:?}")
    disassemble_chunk(&chunk, "test chunk");
    VirtualMachine::interpret(chunk);
}
