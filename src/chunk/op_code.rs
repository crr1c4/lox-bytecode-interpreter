use crate::value::Value;
use std::fmt::Debug;

pub enum OpCode {
    Return,
    Constant(Value),
    Negate,
    Print,
    Not,
    Nil,
    True,
    False,
    Pop,
    Add,
    Substract,
    Multiply,
    Divide,
    Equal,
    Greater,
    Less,
    DefineGlobal,
    GetGlobal,
    SetGlobal,
}

impl Debug for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            OpCode::Return => "OP_RETURN",
            OpCode::Constant(constant) => &format!("{: <16} '{}'", "OP_CONSTANT", constant),
            OpCode::Add => "OP_ADD",
            OpCode::Substract => "OP_SUBSTRACT",
            OpCode::Multiply => "OP_MULTIPLY",
            OpCode::Divide => "OP_DIVIDE",
            OpCode::Negate => "OP_NEGATE",
            OpCode::Nil => "OP_NIL",
            OpCode::True => "OP_TRUE",
            OpCode::False => "OP_FALSE",
            OpCode::Not => "OP_NOT",
            OpCode::Equal => "OP_EQUAL",
            OpCode::Greater => "OP_GREATER",
            OpCode::Less => "OP_LESS",
            OpCode::Print => "OP_PRINT",
            OpCode::Pop => "OP_POP",
            OpCode::DefineGlobal => "OP_DEFINE_GLOBAL",
            OpCode::GetGlobal => "OP_GET_GLOBAL",
            OpCode::SetGlobal => "OP_SET_GLOBAL"
        };

        write!(f, "{}", output)
    }
}
