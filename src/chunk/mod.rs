use super::Value;

#[derive(Debug)]
pub enum OperationCode {
    Return,
    Constant(Value)
}

#[derive(Default, Debug)]
pub struct Chunk {
    codes: Vec<OperationCode>,
    lines: Vec<u32>
}

impl Chunk {
    pub fn create() -> Self {
        Self::default()
    }

    pub fn write(&mut self, code: OperationCode, line: u32) {
        self.codes.push(code);
        self.lines.push(line);
    }
}
