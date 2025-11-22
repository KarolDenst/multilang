use crate::error::RuntimeError;
use crate::node::Value;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

pub fn read_file_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError {
            message: format!("read_file expects 1 argument, got {}", args.len()),
            stack_trace: vec![],
        });
    }

    match &args[0] {
        Value::String(path) => {
            let path_str = path.borrow();

            match fs::read_to_string(path_str.as_str()) {
                Ok(content) => Ok(Value::String(Rc::new(RefCell::new(content)))),
                Err(e) => Err(RuntimeError {
                    message: format!("read_file: failed to read '{}': {}", path_str, e),
                    stack_trace: vec![],
                }),
            }
        }
        _ => Err(RuntimeError {
            message: "read_file expects a string path".to_string(),
            stack_trace: vec![],
        }),
    }
}
