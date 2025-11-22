use crate::error::RuntimeError;
use crate::node::Value;

pub fn reverse_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError {
            message: format!("reverse expects 1 argument, got {}", args.len()),
            stack_trace: vec![],
        });
    }

    match &args[0] {
        Value::List(l) => {
            l.borrow_mut().reverse();
            Ok(Value::Void)
        }
        _ => Err(RuntimeError {
            message: "reverse expects a list".to_string(),
            stack_trace: vec![],
        }),
    }
}
