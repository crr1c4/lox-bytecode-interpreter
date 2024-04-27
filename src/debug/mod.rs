use crate::chunk::Chunk;
use crate::chunk::OperationCode;
use crate::value::Value;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset = 0u32;

    while offset < chunk.count() as u32 {
        offset = disassemble_instruction(chunk, offset);
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: u32) -> u32 {
    print!("{:04} ", offset);

    if offset > 0 && chunk.lines[offset as usize] == chunk.lines[offset as usize - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset as usize]);
    }

    let instruction = chunk.codes.get(offset as usize).unwrap();

    match instruction {
        OperationCode::Return => simple_instruction("OP_RETURN", offset),
        OperationCode::Constant(constant) => constant_instruction("OP_CONSTANT", *constant, offset),
        OperationCode::Add => simple_instruction("OP_ADD", offset),
        OperationCode::Substract => simple_instruction("OP_SUBSTRACT", offset),
        OperationCode::Multiply => simple_instruction("OP_MULTIPLY", offset),
        OperationCode::Divide => simple_instruction("OP_DIVIDE", offset),
        OperationCode::Negate => simple_instruction("OP_NEGATE", offset),
        /* _ => {
            println!("Unknown operation code {}", offset);
            offset + 1
        } */
    }
}

pub fn simple_instruction(name: &str, offset: u32) -> u32 {
    println!("{}", name);
    offset + 1
}

fn constant_instruction(name: &str, constant: Value, offset: u32) -> u32 {
    println!("{: <16} '{}'", name, constant);
    offset + 1
}

pub fn print_value(value: Value) {
    println!("{:?}", value);
}
