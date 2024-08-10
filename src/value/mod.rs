// BUG: Add error messages for Object

pub mod object;

use derive_more::derive::Display;
use derive_more::derive::Debug;
use crate::value::object::Object;

#[derive(Clone, Default, Display, Debug)]
#[display("{_0}")]
pub enum Value {
    Bool(bool),
    #[default]
    #[display("Nil")]
    #[debug("Nil")]
    Nil,
    Number(f64),
    #[debug("{_0}")]
    Object(Box<dyn Object>),
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::Object(Box::new(value))
    }
}
