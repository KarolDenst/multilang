use crate::error::RuntimeError;
use crate::node::Value;

pub fn to_int_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError {
            message: format!("to_int expects 1 argument, got {}", args.len()),
            stack_trace: vec![],
        });
    }

    match &args[0] {
        Value::String(s) => {
            let s = s.borrow();
            let trimmed = s.trim();
            match trimmed.parse::<i32>() {
                Ok(i) => Ok(Value::Int(i)),
                Err(_) => Err(RuntimeError {
                    message: format!("to_int: invalid number format '{}'", trimmed),
                    stack_trace: vec![],
                }),
            }
        }
        Value::Int(i) => Ok(Value::Int(*i)),
        Value::Float(f) => Ok(Value::Int(*f as i32)),
        _ => Err(RuntimeError {
            message: "to_int expects a string, int, or float".to_string(),
            stack_trace: vec![],
        }),
    }
}
