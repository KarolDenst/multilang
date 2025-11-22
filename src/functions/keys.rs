use crate::error::RuntimeError;
use crate::node::Value;
use std::cell::RefCell;
use std::rc::Rc;

pub fn keys_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError {
            message: format!("keys expects 1 argument, got {}", args.len()),
            stack_trace: vec![],
        });
    }
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let keys: Vec<Value> = map
                .keys()
                .map(|k| Value::String(Rc::new(RefCell::new(k.clone()))))
                .collect();
            Ok(Value::List(Rc::new(RefCell::new(keys))))
        }
        _ => Err(RuntimeError {
            message: "First argument to keys must be a map".to_string(),
            stack_trace: vec![],
        }),
    }
}
