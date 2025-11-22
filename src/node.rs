use std::collections::HashMap;

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::RuntimeError;
use crate::grammar::Rule;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f64),
    String(Rc<RefCell<String>>),
    Bool(bool),
    List(Rc<RefCell<Vec<Value>>>),
    Map(Rc<RefCell<HashMap<String, Value>>>),
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

        ctx.builtins.insert("get".to_string(), |args| {
            if args.len() != 2 {
                return Err(RuntimeError {
                    message: format!("get expects 2 arguments, got {}", args.len()),
                    stack_trace: vec![],
                });
            }
            match &args[0] {
                Value::List(l) => {
                    let list = l.borrow();
                    let index = match &args[1] {
                        Value::Int(i) => *i as usize,
                        _ => {
                            return Err(RuntimeError {
                                message: "Second argument to get for List must be an integer"
                                    .to_string(),
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
                }
                Value::Map(m) => {
                    let map = m.borrow();
                    let key = match &args[1] {
                        Value::String(s) => s,
                        _ => {
                            return Err(RuntimeError {
                                message: "Second argument to get for Map must be a string"
                                    .to_string(),
                                stack_trace: vec![],
                            });
                        }
                    };
                    match map.get(key.borrow().as_str()) {
                        Some(val) => Ok(val.clone()),
                        None => Ok(Value::Void), // Or error? Void seems safer for now
                    }
                }
                Value::String(s) => {
                    let string = s.borrow();
                    let index = match &args[1] {
                        Value::Int(i) => *i as usize,
                        _ => {
                            return Err(RuntimeError {
                                message: "Second argument to get for String must be an integer"
                                    .to_string(),
                                stack_trace: vec![],
                            });
                        }
                    };
                    if let Some(ch) = string.chars().nth(index) {
                        Ok(Value::String(Rc::new(RefCell::new(ch.to_string()))))
                    } else {
                        Err(RuntimeError {
                            message: format!(
                                "Index {} out of bounds (len {})",
                                index,
                                string.len()
                            ),
                            stack_trace: vec![],
                        })
                    }
                }
                _ => Err(RuntimeError {
                    message: "First argument to get must be a list, map, or string".to_string(),
                    stack_trace: vec![],
                }),
            }
        });

        ctx.builtins.insert("set".to_string(), |args| {
            if args.len() != 3 {
                return Err(RuntimeError {
                    message: format!("set expects 3 arguments, got {}", args.len()),
                    stack_trace: vec![],
                });
            }
            match &args[0] {
                Value::Map(m) => {
                    let mut map = m.borrow_mut();
                    let key = match &args[1] {
                        Value::String(s) => s.borrow().clone(),
                        _ => {
                            return Err(RuntimeError {
                                message: "Second argument to set for Map must be a string"
                                    .to_string(),
                                stack_trace: vec![],
                            });
                        }
                    };
                    map.insert(key, args[2].clone());
                    Ok(Value::Void)
                }
                Value::List(l) => {
                    let mut list = l.borrow_mut();
                    let index = match &args[1] {
                        Value::Int(i) => *i as usize,
                        _ => {
                            return Err(RuntimeError {
                                message: "Second argument to set for List must be an integer"
                                    .to_string(),
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
                    list[index] = args[2].clone();
                    Ok(Value::Void)
                }
                Value::String(s) => {
                    let mut string = s.borrow_mut();
                    let index = match &args[1] {
                        Value::Int(i) => *i as usize,
                        _ => {
                            return Err(RuntimeError {
                                message: "Second argument to set for String must be an integer"
                                    .to_string(),
                                stack_trace: vec![],
                            });
                        }
                    };
                    let char_val = match &args[2] {
                        Value::String(c) => c.borrow().clone(),
                        _ => {
                            return Err(RuntimeError {
                                message: "Third argument to set for String must be a string"
                                    .to_string(),
                                stack_trace: vec![],
                            });
                        }
                    };

                    // This is a bit tricky with UTF-8.
                    // We need to replace the nth char.
                    // Simplest way: collect chars, replace, collect string.
                    let mut chars: Vec<char> = string.chars().collect();
                    if index >= chars.len() {
                        return Err(RuntimeError {
                            message: format!("Index {} out of bounds (len {})", index, chars.len()),
                            stack_trace: vec![],
                        });
                    }

                    // We expect char_val to be a single char string, or we take the first char?
                    // Let's enforce single char for now or just take the first one.
                    if let Some(c) = char_val.chars().next() {
                        chars[index] = c;
                        *string = chars.into_iter().collect();
                        Ok(Value::Void)
                    } else {
                        Err(RuntimeError {
                            message: "Value to set in string cannot be empty".to_string(),
                            stack_trace: vec![],
                        })
                    }
                }
                _ => Err(RuntimeError {
                    message: "First argument to set must be a map, list, or string".to_string(),
                    stack_trace: vec![],
                }),
            }
        });

        ctx.builtins.insert("keys".to_string(), |args| {
            if args.len() != 1 {
                return Err(RuntimeError {
                    message: format!("keys expects 1 argument, got {}", args.len()),
                    stack_trace: vec![],
                });
            }
            match &args[0] {
                Value::Map(m) => {
                    let map = m.borrow();
                    let keys: Vec<Value> = map
                        .keys()
                        .map(|k| Value::String(Rc::new(RefCell::new(k.clone()))))
                        .collect();
                    Ok(Value::List(Rc::new(RefCell::new(keys))))
                }
                _ => Err(RuntimeError {
                    message: "First argument to keys must be a map".to_string(),
                    stack_trace: vec![],
                }),
            }
        });

        ctx.builtins.insert("append".to_string(), |args| {
            if args.len() != 2 {
                return Err(RuntimeError {
                    message: format!("append expects 2 arguments, got {}", args.len()),
                    stack_trace: vec![],
                });
            }
            match &args[0] {
                Value::List(list) => {
                    list.borrow_mut().push(args[1].clone());
                    Ok(Value::Void)
                }
                Value::String(s) => {
                    let mut string = s.borrow_mut();
                    match &args[1] {
                        Value::String(other) => {
                            string.push_str(&other.borrow());
                            Ok(Value::Void)
                        }
                        _ => Err(RuntimeError {
                            message: "Second argument to append for String must be a string"
                                .to_string(),
                            stack_trace: vec![],
                        }),
                    }
                }
                _ => Err(RuntimeError {
                    message: "First argument to append must be a list or string".to_string(),
                    stack_trace: vec![],
                }),
            }
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
    fn is_map_entries(&self) -> bool {
        false
    }
    fn into_map_entries(self: Box<Self>) -> Vec<(String, Box<dyn Node>)> {
        vec![]
    }

    // Static method to construct node from children
    fn from_children(rule: Rule, children: ParsedChildren) -> Box<dyn Node>
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
