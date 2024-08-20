use crate::opcode::OpCode;
use crate::Line;
use derive_more::Debug;

#[derive(Debug)]
#[debug("{:04}\t{:?}", _1, _0)]
pub struct Code(pub OpCode, pub Line);

#[derive(Debug)]
#[debug("Chunk {:p} {:#?}", self, self.codes)]
pub struct Chunk {
    pub codes: Vec<Code>,
}

impl Chunk {
    pub fn new() -> Self {
        Self { codes: vec![] }
    }

    pub fn write(&mut self, opcode: OpCode, line: Line) {
        self.codes.push(Code(opcode, line));
    }
}
