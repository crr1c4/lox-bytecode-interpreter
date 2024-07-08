use crate::value::Value;
use std::fmt::Debug;

pub enum OperationCode {
    Return,
    Constant(Value),
    Negate,
    Not,
    Nil,
    True,
    False,
    Add,
    Substract,
    Multiply,
    Divide,
    Equal,
    Greater,
    Less
}

impl Debug for OperationCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            OperationCode::Return => "OP_RETURN",
            OperationCode::Constant(constant) => &format!("{: <16} '{}'", "OP_CONSTANT", constant),
            OperationCode::Add => "OP_ADD",
            OperationCode::Substract => "OP_SUBSTRACT",
            OperationCode::Multiply => "OP_MULTIPLY",
            OperationCode::Divide => "OP_DIVIDE",
            OperationCode::Negate => "OP_NEGATE",
            OperationCode::Nil => "OP_NIL",
            OperationCode::True => "OP_TRUE",
            OperationCode::False => "OP_FALSE",
            OperationCode::Not => "OP_NOT",
            OperationCode::Equal => "OP_EQUAL",
            OperationCode::Greater => "OP_GREATER",
            OperationCode::Less => "OP_LESS"
        };

        write!(f, "{}", output)
    }
}
