use thiserror::Error;
use std::path::Path;
use crate::Line;
use crate::Identifier;

// #[derive(PartialEq, Error, Debug)]
// pub enum CompileError {
//     MissingSemicolon,
//     MissingBrace
// }


#[derive(PartialEq, Error, Debug)]
pub enum InputError {
    #[error("Failed to read line")]
    ReadLine(),
    #[error("Failed to read {0}.")]
    FileNotFound(Box<Path>)
}

#[derive(PartialEq, Error, Debug)]
pub enum RuntimeError {
    #[error("Operands must be a number. [line {0}] in script.")]
    ExpectedNumber(Line),
    #[error("Operands must be a number or string. [line {0}] in script.")]
    ExpectedNumberOrString(Line),
    #[error("Exepected value. [line {0}] in script.")]
    ExpectedValue(Line),
    #[error("Undefined variable '{0}'. [line {1}] in script.")]
    UndefinedVariable(Identifier, Line),
}
