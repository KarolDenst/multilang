use crate::error::RuntimeError;
use crate::node::Value;
use std::cmp::Ordering;

pub fn sort_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError {
            message: format!("sort expects 1 argument, got {}", args.len()),
            stack_trace: vec![],
        });
    }

    match &args[0] {
        Value::List(l) => {
            let mut list = l.borrow_mut();

            // Check if all elements are comparable
            if list.is_empty() {
                return Ok(Value::Void);
            }

            // Sort with custom comparator
            list.sort_by(|a, b| compare_values(a, b).unwrap_or(Ordering::Equal));

            Ok(Value::Void)
        }
        _ => Err(RuntimeError {
            message: "sort expects a list".to_string(),
            stack_trace: vec![],
        }),
    }
}

fn compare_values(a: &Value, b: &Value) -> Result<Ordering, RuntimeError> {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => Ok(x.cmp(y)),
        (Value::Float(x), Value::Float(y)) => {
            if x < y {
                Ok(Ordering::Less)
            } else if x > y {
                Ok(Ordering::Greater)
            } else {
                Ok(Ordering::Equal)
            }
        }
        (Value::Int(x), Value::Float(y)) => {
            let x_f = *x as f64;
            if x_f < *y {
                Ok(Ordering::Less)
            } else if x_f > *y {
                Ok(Ordering::Greater)
            } else {
                Ok(Ordering::Equal)
            }
        }
        (Value::Float(x), Value::Int(y)) => {
            let y_f = *y as f64;
            if x < &y_f {
                Ok(Ordering::Less)
            } else if x > &y_f {
                Ok(Ordering::Greater)
            } else {
                Ok(Ordering::Equal)
            }
        }
        (Value::String(x), Value::String(y)) => Ok(x.borrow().cmp(&y.borrow())),
        (Value::Bool(x), Value::Bool(y)) => Ok(x.cmp(y)),
        _ => Err(RuntimeError {
            message: "sort: unable to compare mixed types".to_string(),
            stack_trace: vec![],
        }),
    }
}
