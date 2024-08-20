pub mod object;

use crate::value::object::Object;
use derive_more::derive::Debug;
use derive_more::derive::Display;

#[derive(Clone, Default, Display, Debug)]
#[display("{_0}")]
pub enum Value {
    Bool(bool),
    #[display("Nil")]
    #[debug("Nil")]
    #[default]
    Nil,
    Number(f64),
    #[debug("{}", _0)]
    Object(Object),
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
        Self::Object(Object::Str(value))
    }
}
