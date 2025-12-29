use crate::lox::Identifier;
use crate::lox::Line;
use std::path::Path;
use thiserror::Error;

// #[derive(PartialEq, Error, Debug)]
// pub enum CompileError {
//     MissingSemicolon,
//     MissingBrace
// }

#[derive(thiserror::Error, Debug)]
pub enum LexicalError {
    #[error("[line {0}] Unterminated string.")]
    UnterminatedString(Line),
    #[error("[line {0}] Unexpected character.")]
    UnexpectedCharacter(Line),
    #[error("[line {0}] Invalid number format.")]
    InvalidNumberFormat(Line),
}

#[derive(thiserror::Error, Debug)]
pub enum SyntaxError {
    #[error("[line {0}] Expect ')' after expression.")]
    ExpectedRightParen(Line),
}

#[derive(PartialEq, Error, Debug)]
pub enum InputError {
    #[error("Failed to read line")]
    ReadLine(),
    #[error("Failed to read {0}.")]
    FileNotFound(Box<Path>),
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
