use crate::error::RuntimeError;
use crate::node::Value;

pub fn to_float_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError {
            message: format!("to_float expects 1 argument, got {}", args.len()),
            stack_trace: vec![],
        });
    }

    match &args[0] {
        Value::String(s) => {
            let s = s.borrow();
            let trimmed = s.trim();
            match trimmed.parse::<f64>() {
                Ok(f) => Ok(Value::Float(f)),
                Err(_) => Err(RuntimeError {
                    message: format!("to_float: invalid number format '{}'", trimmed),
                    stack_trace: vec![],
                }),
            }
        }
        Value::Int(i) => Ok(Value::Float(*i as f64)),
        Value::Float(f) => Ok(Value::Float(*f)),
        _ => Err(RuntimeError {
            message: "to_float expects a string, int, or float".to_string(),
            stack_trace: vec![],
        }),
    }
}
