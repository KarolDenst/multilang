use crate::error::RuntimeError;
use crate::node::Value;

pub fn set_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
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
                        message: "Second argument to set for Map must be a string".to_string(),
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
                        message: "Second argument to set for List must be an integer".to_string(),
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
                        message: "Second argument to set for String must be an integer".to_string(),
                        stack_trace: vec![],
                    });
                }
            };
            let char_val = match &args[2] {
                Value::String(c) => c.borrow().clone(),
                _ => {
                    return Err(RuntimeError {
                        message: "Third argument to set for String must be a string".to_string(),
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
}
