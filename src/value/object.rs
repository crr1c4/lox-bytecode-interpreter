use derive_more::Display;

#[derive(Clone, Display)]
pub enum Object {
    #[display("{_0}")]
    Str(String),
}
