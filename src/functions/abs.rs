use crate::error::RuntimeError;
use crate::node::Value;

pub fn abs_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError {
            message: format!("abs expects 1 argument, got {}", args.len()),
            stack_trace: vec![],
        });
    }

    match &args[0] {
        Value::Int(n) => Ok(Value::Int(n.abs())),
        Value::Float(n) => Ok(Value::Float(n.abs())),
        _ => Err(RuntimeError {
            message: "abs expects a number (int or float)".to_string(),
            stack_trace: vec![],
        }),
    }
}
