use crate::error::RuntimeError;
use crate::node::Value;
use std::cell::RefCell;
use std::rc::Rc;

pub fn split_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError {
            message: format!("split expects 2 arguments, got {}", args.len()),
            stack_trace: vec![],
        });
    }

    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(delim)) => {
            let string = s.borrow();
            let delimiter = delim.borrow();

            let parts: Vec<Value> = string
                .split(delimiter.as_str())
                .map(|part| Value::String(Rc::new(RefCell::new(part.to_string()))))
                .collect();

            Ok(Value::List(Rc::new(RefCell::new(parts))))
        }
        _ => Err(RuntimeError {
            message: "split expects two strings (string, delimiter)".to_string(),
            stack_trace: vec![],
        }),
    }
}
