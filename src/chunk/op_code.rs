use crate::value::Value;
use std::fmt::Debug;

pub enum OperationCode {
    Return,
    Constant(Value),
    Negate,
    Add,
    Substract,
    Multiply,
    Divide,
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
        };

        write!(f, "{}", output)
    }
}
