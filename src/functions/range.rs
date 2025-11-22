use crate::error::RuntimeError;
use crate::node::Value;
use std::cell::RefCell;
use std::rc::Rc;

pub fn range_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.is_empty() || args.len() > 2 {
        return Err(RuntimeError {
            message: format!("range expects 1 or 2 arguments, got {}", args.len()),
            stack_trace: vec![],
        });
    }

    let (start, end) = if args.len() == 1 {
        match &args[0] {
            Value::Int(n) => (0, *n),
            _ => {
                return Err(RuntimeError {
                    message: "range expects integer arguments".to_string(),
                    stack_trace: vec![],
                });
            }
        }
    } else {
        match (&args[0], &args[1]) {
            (Value::Int(s), Value::Int(e)) => (*s, *e),
            _ => {
                return Err(RuntimeError {
                    message: "range expects integer arguments".to_string(),
                    stack_trace: vec![],
                });
            }
        }
    };

    let mut result = Vec::new();
    for i in start..end {
        result.push(Value::Int(i));
    }

    Ok(Value::List(Rc::new(RefCell::new(result))))
}
