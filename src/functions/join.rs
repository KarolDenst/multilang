use crate::error::RuntimeError;
use crate::node::Value;
use std::cell::RefCell;
use std::rc::Rc;

pub fn join_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError {
            message: format!("join expects 2 arguments, got {}", args.len()),
            stack_trace: vec![],
        });
    }

    match (&args[0], &args[1]) {
        (Value::List(l), Value::String(delim)) => {
            let list = l.borrow();
            let delimiter = delim.borrow();

            let strings: Result<Vec<String>, RuntimeError> = list
                .iter()
                .map(|val| match val {
                    Value::String(s) => Ok(s.borrow().clone()),
                    Value::Int(n) => Ok(n.to_string()),
                    Value::Float(n) => Ok(n.to_string()),
                    Value::Bool(b) => Ok(b.to_string()),
                    _ => Err(RuntimeError {
                        message: "join: list elements must be strings or convertible to strings"
                            .to_string(),
                        stack_trace: vec![],
                    }),
                })
                .collect();

            let strings = strings?;
            let result = strings.join(&delimiter);

            Ok(Value::String(Rc::new(RefCell::new(result))))
        }
        _ => Err(RuntimeError {
            message: "join expects a list and a string delimiter".to_string(),
            stack_trace: vec![],
        }),
    }
}
