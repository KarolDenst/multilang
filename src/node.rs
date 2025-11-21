use std::collections::HashMap;

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::RuntimeError;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
    List(Rc<RefCell<Vec<Value>>>),
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

        ctx.builtins.insert("append".to_string(), |args| {
            if args.len() != 2 {
                return Err(RuntimeError {
                    message: format!("append expects 2 arguments, got {}", args.len()),
                    stack_trace: vec![],
                });
            }
            if let Value::List(list) = &args[0] {
                list.borrow_mut().push(args[1].clone());
                Ok(Value::Void)
            } else {
                Err(RuntimeError {
                    message: "First argument to append must be a list".to_string(),
                    stack_trace: vec![],
                })
            }
        });

        ctx.builtins.insert("get".to_string(), |args| {
            if args.len() != 2 {
                return Err(RuntimeError {
                    message: format!("get expects 2 arguments, got {}", args.len()),
                    stack_trace: vec![],
                });
            }
            let list = match &args[0] {
                Value::List(l) => l.borrow(),
                _ => {
                    return Err(RuntimeError {
                        message: "First argument to get must be a list".to_string(),
                        stack_trace: vec![],
                    });
                }
            };

            let index = match &args[1] {
                Value::Int(i) => *i as usize,
                _ => {
                    return Err(RuntimeError {
                        message: "Second argument to get must be an integer".to_string(),
                        stack_trace: vec![],
                    });
                }
            };

            if index >= list.len() {
                return Err(RuntimeError {
                    message: format!("Index {} out of bounds (len {})", index, list.len()),
                    stack_trace: vec![],
                });
            }

            Ok(list[index].clone())
        });

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
    fn is_list_elements(&self) -> bool {
        false
    }
    fn into_list_elements(self: Box<Self>) -> Vec<Box<dyn Node>> {
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
