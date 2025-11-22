use crate::error::RuntimeError;
use crate::node::Value;
use std::cell::RefCell;
use std::rc::Rc;

pub fn chr_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError {
            message: format!("chr expects 1 argument, got {}", args.len()),
            stack_trace: vec![],
        });
    }

    match &args[0] {
        Value::Int(n) => {
            if *n < 0 || *n > 1114111 {
                return Err(RuntimeError {
                    message: format!(
                        "chr expects a valid Unicode code point (0-1114111), got {}",
                        n
                    ),
                    stack_trace: vec![],
                });
            }

            match char::from_u32(*n as u32) {
                Some(c) => Ok(Value::String(Rc::new(RefCell::new(c.to_string())))),
                None => Err(RuntimeError {
                    message: format!("Invalid Unicode code point: {}", n),
                    stack_trace: vec![],
                }),
            }
        }
        _ => Err(RuntimeError {
            message: "chr expects an integer".to_string(),
            stack_trace: vec![],
        }),
    }
}
