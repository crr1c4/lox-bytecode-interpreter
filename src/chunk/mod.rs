pub mod op_code;

use op_code::OpCode;
use std::fmt::Debug;
use std::ops::Deref;

pub type Line = u32;

#[derive(Default)]
pub struct Chunk {
    codes: Vec<(OpCode, Line)>,
}

impl Deref for Chunk {
    type Target = Vec<(OpCode, Line)>;

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

    pub fn write(&mut self, code: OpCode, line: Line) {
        self.codes.push((code, line));
    }
}
