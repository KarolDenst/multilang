use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Bool(bool),
    Void,
}

pub struct Context {
    // For now, context can be empty or hold variables later
    pub variables: HashMap<String, Value>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}

pub trait Node {
    fn run(&self, ctx: &mut Context) -> Value;
    fn text(&self) -> Option<String> { None }
}
