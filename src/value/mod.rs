use std::fmt::Display;

// pub type Value = f64;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(f64)
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Value::Number(number) => format!("{number}"),
            Value::Bool(boolean) => format!("{boolean}"),
            Value::Nil => format!("Nil")
        };

        write!(f, "{}", value)
    }
}
