use crate::error::RuntimeError;
use crate::node::Value;

pub fn len_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError {
            message: format!("len expects 1 argument, got {}", args.len()),
            stack_trace: vec![],
        });
    }

    match &args[0] {
        Value::String(s) => Ok(Value::Int(s.borrow().len() as i32)),
        Value::List(l) => Ok(Value::Int(l.borrow().len() as i32)),
        Value::Map(m) => Ok(Value::Int(m.borrow().len() as i32)),
        _ => Err(RuntimeError {
            message: "len expects a string, list, or map".to_string(),
            stack_trace: vec![],
        }),
    }
}
