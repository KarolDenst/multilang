use crate::error::RuntimeError;
use crate::node::Value;

pub fn ord_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError {
            message: format!("ord expects 1 argument, got {}", args.len()),
            stack_trace: vec![],
        });
    }

    match &args[0] {
        Value::String(s) => {
            let str_val = s.borrow();
            let chars: Vec<char> = str_val.chars().collect();

            if chars.len() != 1 {
                return Err(RuntimeError {
                    message: format!(
                        "ord expects a single character, got string of length {}",
                        chars.len()
                    ),
                    stack_trace: vec![],
                });
            }

            Ok(Value::Int(chars[0] as i32))
        }
        _ => Err(RuntimeError {
            message: "ord expects a string".to_string(),
            stack_trace: vec![],
        }),
    }
}
