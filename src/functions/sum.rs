use crate::error::RuntimeError;
use crate::node::Value;

pub fn sum_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError {
            message: format!("sum expects 1 argument, got {}", args.len()),
            stack_trace: vec![],
        });
    }

    match &args[0] {
        Value::List(l) => {
            let list = l.borrow();
            let mut int_sum = 0i32;
            let mut float_sum = 0.0f64;
            let mut has_float = false;

            for item in list.iter() {
                match item {
                    Value::Int(n) => {
                        if has_float {
                            float_sum += *n as f64;
                        } else {
                            int_sum += n;
                        }
                    }
                    Value::Float(n) => {
                        if !has_float {
                            has_float = true;
                            float_sum = int_sum as f64 + n;
                        } else {
                            float_sum += n;
                        }
                    }
                    _ => {
                        return Err(RuntimeError {
                            message: "sum expects a list of numbers".to_string(),
                            stack_trace: vec![],
                        });
                    }
                }
            }

            if has_float {
                Ok(Value::Float(float_sum))
            } else {
                Ok(Value::Int(int_sum))
            }
        }
        _ => Err(RuntimeError {
            message: "sum expects a list".to_string(),
            stack_trace: vec![],
        }),
    }
}
