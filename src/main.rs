use lox_interpreter::chunk::*;

fn main() {
    let mut chunk = Chunk::create();
    chunk.write(OperationCode::Constant(1.2), 123);
    chunk.write(OperationCode::Constant(3.14), 123);
    chunk.write(OperationCode::Return, 124);
    println!("{chunk:?}")
}
