use crate::error::RuntimeError;
use crate::node::Value;

pub fn append_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
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
                    message: "Second argument to append for String must be a string".to_string(),
                    stack_trace: vec![],
                }),
            }
        }
        _ => Err(RuntimeError {
            message: "First argument to append must be a list or string".to_string(),
            stack_trace: vec![],
        }),
    }
}
