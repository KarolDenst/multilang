use crate::error::RuntimeError;
use crate::node::Value;
use std::cell::RefCell;
use std::rc::Rc;

pub fn get_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
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
                        message: "Second argument to get for List must be an integer".to_string(),
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
                        message: "Second argument to get for Map must be a string".to_string(),
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
                        message: "Second argument to get for String must be an integer".to_string(),
                        stack_trace: vec![],
                    });
                }
            };
            if let Some(ch) = string.chars().nth(index) {
                Ok(Value::String(Rc::new(RefCell::new(ch.to_string()))))
            } else {
                Err(RuntimeError {
                    message: format!("Index {} out of bounds (len {})", index, string.len()),
                    stack_trace: vec![],
                })
            }
        }
        _ => Err(RuntimeError {
            message: "First argument to get must be a list, map, or string".to_string(),
            stack_trace: vec![],
        }),
    }
}
