pub mod op_code;

use op_code::OpCode;
use std::fmt::Debug;
use std::ops::Deref;

#[derive(Default)]
pub struct Chunk {
    // NOTE: u32 field in the tuple refers to the line of the operation code.
    pub codes: Vec<(OpCode, u32)>,
}

impl Deref for Chunk {
    type Target = Vec<(OpCode, u32)>;

    fn deref(&self) -> &Self::Target {
        &self.codes
    }
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "--- Chunk {:p} ---", self)?;

        for (index, (code, line)) in self.codes.iter().enumerate() {
            write!(f, "{:04} ", index)?;
            
            let code_index = if index > 0 { index - 1 } else { index };

            match self.codes.get(code_index) {
                Some((_, previous)) if index > 0 && line == previous => write!(f, "   | ")?,
                _ => write!(f, "{:4} ", line)?,
            };

            writeln!(f, "{:?}", code)?;
        }

        write!(f, "--- --- --- --- --- --- ---")
    }
}

impl Chunk {
    pub fn create() -> Self {
        Self::default()
    }

    pub fn write(&mut self, code: OpCode, line: u32) {
        self.codes.push((code, line));
    }
}
