use std::collections::HashMap;

use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Bool(bool),
    Void,
}

#[derive(Clone)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Rc<dyn Node>,
}

pub struct Context {
    // For now, context can be empty or hold variables later
    pub variables: HashMap<String, Value>,
    pub functions: HashMap<String, Function>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }
}

pub trait Node {
    fn run(&self, ctx: &mut Context) -> Value;
    fn text(&self) -> Option<String> { None }
    fn params(&self) -> Option<Vec<String>> { None }
    fn is_args(&self) -> bool { false }
    fn into_args(self: Box<Self>) -> Vec<Box<dyn Node>> { vec![] }
}
