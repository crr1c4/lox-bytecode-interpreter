use derive_more::Debug;
use derive_more::Display;

#[derive(Clone, Default, Display, Debug)]
#[display("{_0}")]
pub enum Value {
    Bool(bool),
    #[display("Nil")]
    #[debug("Nil")]
    #[default]
    Nil,
    Number(f64),
    String(String),
}

// impl From<bool> for Value {
//     fn from(value: bool) -> Self {
//         Self::Bool(value)
//     }
// }

// impl From<f64> for Value {
//     fn from(value: f64) -> Self {
//         Self::Number(value)
//     }
// }

// impl From<String> for Value {
//     fn from(value: String) -> Self {
//         Self::String(value)
//     }
// }
// #[derive(Debug)]
// pub enum Expression {
//     #[debug("({operator} {left:?} {right:?} )")]
//     Binary {
//         left: Box<Expression>,
//         operator: Token,
//         right: Box<Expression>,
//     },
//     #[debug("(grouping {expression:?})")]
//     Grouping { expression: Box<Expression> },
//     #[debug("{value}")]
//     Literal { value: Value },
//     #[debug("({operator} {right:?})")]
//     Unary { right: Box<Expression>, operator: Token },
// }
