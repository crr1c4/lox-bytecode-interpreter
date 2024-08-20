use crate::value::Value;
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
    #[debug("{: <16} {}", "OP_DEFINE_GLOBAL", _0)]
    DefineGlobal(String),
    #[debug("{: <16} {}", "OP_GET_GLOBAL", _0)]
    GetGlobal(String),
    #[debug("{: <16} {}", "OP_SET_GLOBAL", _0)]
    SetGlobal(String),
}
