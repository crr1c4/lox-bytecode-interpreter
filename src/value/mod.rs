// BUG: Add error messages for Object
// TODO: Add a linked list for Object values (page 352)

pub mod object;

use crate::value::object::Object;
use std::fmt::Display;

#[derive(Clone, Default)]
pub enum Value {
    Bool(bool),
    #[default]
    Nil,
    Number(f64),
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

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Value::Number(number) => format!("{number}"),
            Value::Bool(boolean) => format!("{boolean}"),
            Value::Nil => format!("Nil"),
            Value::Object(value) => format!("{value}"),
        };

        write!(f, "{}", value)
    }
}
