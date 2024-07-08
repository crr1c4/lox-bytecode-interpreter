pub mod object;

use std::fmt::Display;
use crate::value::object::Object;

#[derive(Clone)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(f64),
    Object(Object)
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Value::Number(number) => format!("{number}"),
            Value::Bool(boolean) => format!("{boolean}"),
            Value::Nil => format!("Nil"),
            Value::Object(Object::String(value)) => format!("{value}") 
        };

        write!(f, "{}", value)
    }
}
