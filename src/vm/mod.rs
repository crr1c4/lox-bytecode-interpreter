use super::chunk::Chunk;

struct VirtualMachine {

}

impl VirtualMachine {
    pub fn interpret(chunk: Chunk) -> InterpretResult {
        todo!()
    }
}

enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError
}
