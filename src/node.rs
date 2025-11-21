use std::collections::HashMap;

use std::rc::Rc;

use crate::error::RuntimeError;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
    Void,
}

#[derive(Clone)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Rc<dyn Node>,
}

pub type BuiltInFunction = fn(Vec<Value>) -> Result<Value, RuntimeError>;

pub struct Context {
    // For now, context can be empty or hold variables later
    pub variables: HashMap<String, Value>,
    pub functions: HashMap<String, Function>,
    pub builtins: HashMap<String, BuiltInFunction>,
}

impl Context {
    pub fn new() -> Self {
        let mut ctx = Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            builtins: HashMap::new(),
        };
        // Register built-ins
        ctx.builtins
            .insert("print".to_string(), crate::functions::print::print_fn);
        ctx
    }
}

pub trait Node {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError>;
    fn text(&self) -> Option<String> {
        None
    }
    fn params(&self) -> Option<Vec<String>> {
        None
    }
    fn is_args(&self) -> bool {
        false
    }
    fn into_args(self: Box<Self>) -> Vec<Box<dyn Node>> {
        vec![]
    }

    // Static method to construct node from children
    fn from_children(rule_name: &str, children: ParsedChildren) -> Box<dyn Node>
    where
        Self: Sized;

    fn box_clone(&self) -> Box<dyn Node>;
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Box<dyn Node> {
        self.box_clone()
    }
}

pub struct ParsedChildren {
    pub children: Vec<(Option<String>, Box<dyn Node>)>,
    pub line: usize,
}

impl ParsedChildren {
    pub fn new(children: Vec<(Option<String>, Box<dyn Node>)>, line: usize) -> Self {
        Self { children, line }
    }

    pub fn take_child(&mut self, name: &str) -> Option<Box<dyn Node>> {
        // 1. Try named match
        if let Some(pos) = self
            .children
            .iter()
            .position(|(n, _)| n.as_deref() == Some(name))
        {
            return Some(self.children.remove(pos).1);
        }
        // 2. Fallback to first unnamed
        if let Some(pos) = self.children.iter().position(|(n, _)| n.is_none()) {
            return Some(self.children.remove(pos).1);
        }
        None
    }

    pub fn remaining(self) -> Vec<(Option<String>, Box<dyn Node>)> {
        self.children
    }
}
