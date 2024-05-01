use crate::value::Value;

#[cfg(test)]
pub mod tests;

#[derive(Debug)]
pub enum OperationCode {
    Return,
    Constant(Value),
    Negate,
    Add,
    Substract,
    Multiply,
    Divide,
}

#[derive(Default, Debug)]
pub struct Chunk {
    pub codes: Vec<OperationCode>,
    pub lines: Vec<u32>,
}

impl Chunk {
    pub fn create() -> Self {
        Self::default()
    }

    pub fn write(&mut self, code: OperationCode, line: u32) {
        self.codes.push(code);
        self.lines.push(line);
    }

    pub fn get(&self, index: usize) -> Option<&OperationCode> {
        self.codes.get(index)
    }

    pub fn count(&self) -> usize {
        self.codes.len()
    }
}


