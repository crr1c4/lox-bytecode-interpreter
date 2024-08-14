use std::fmt::Display;
// use dyn_clone::DynClone;

pub trait Object: Display {
    fn clone_object(&self) -> Box<dyn Object>;
}

impl Object for String {
    fn clone_object(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Object> {
    fn clone(&self) -> Self {
        self.clone_object()
    }
}
