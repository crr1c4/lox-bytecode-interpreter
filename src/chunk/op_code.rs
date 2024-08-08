use crate::value::Value;
// use std::fmt::Debug;
use derive_more::Debug;

#[derive(Debug)]
pub enum OpCode {
    #[debug("OP_RETURN")]
    Return,
    #[debug("{: <16} {}", "OP_CONSTANT", _0)]
    Constant(Value),
    #[debug("OP_NEGATE")]
    Negate,
    #[debug("OP_PRINT")]
    Print,
    #[debug("OP_NOT")]
    Not,
    #[debug("OP_NIL")]
    Nil,
    #[debug("OP_TRUE")]
    True,
    #[debug("OP_FALSE")]
    False,
    #[debug("OP_POP")]
    Pop,
    #[debug("OP_ADD")]
    Add,
    #[debug("OP_SUBSTRACT")]
    Substract,
    #[debug("OP_MULTIPLY")]
    Multiply,
    #[debug("OP_DIVIDE")]
    Divide,
    #[debug("OP_EQUAL")]
    Equal,
    #[debug("OP_GREATER")]
    Greater,
    #[debug("OP_LESS")]
    Less,
    #[debug("OP_DEFINE_GLOBAL")]
    DefineGlobal,
    #[debug("OP_GET_GLOBAL")]
    GetGlobal,
    #[debug("OP_SET_GLOBAL")]
    SetGlobal,
}

// impl Debug for OpCode {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let output = match self {
//             OpCode::Return => "OP_RETURN",
//             OpCode::Constant(constant) => &format!("{: <16} '{}'", "OP_CONSTANT", constant),
//             OpCode::Add => "OP_ADD",
//             OpCode::Substract => "OP_SUBSTRACT",
//             OpCode::Multiply => "OP_MULTIPLY",
//             OpCode::Divide => "OP_DIVIDE",
//             OpCode::Negate => "OP_NEGATE",
//             OpCode::Nil => "OP_NIL",
//             OpCode::True => "OP_TRUE",
//             OpCode::False => "OP_FALSE",
//             OpCode::Not => "OP_NOT",
//             OpCode::Equal => "OP_EQUAL",
//             OpCode::Greater => "OP_GREATER",
//             OpCode::Less => "OP_LESS",
//             OpCode::Print => "OP_PRINT",
//             OpCode::Pop => "OP_POP",
//             OpCode::DefineGlobal => "OP_DEFINE_GLOBAL",
//             OpCode::GetGlobal => "OP_GET_GLOBAL",
//             OpCode::SetGlobal => "OP_SET_GLOBAL"
//         };
//
//         write!(f, "{}", output)
//     }
// }
