use crate::error::RuntimeError;
use crate::node::Value;
use std::cell::RefCell;
use std::rc::Rc;

pub fn slice_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError {
            message: format!("slice expects 3 arguments, got {}", args.len()),
            stack_trace: vec![],
        });
    }

    let (start_idx, end_idx) = match (&args[1], &args[2]) {
        (Value::Int(s), Value::Int(e)) => (*s, *e),
        _ => {
            return Err(RuntimeError {
                message: "slice expects integer indices for start and end".to_string(),
                stack_trace: vec![],
            });
        }
    };

    match &args[0] {
        Value::String(s) => {
            let string = s.borrow();
            let chars: Vec<char> = string.chars().collect();
            let len = chars.len() as i32;

            let start = normalize_index(start_idx, len);
            let end = normalize_index(end_idx, len);

            if start < 0 || end < 0 || start > len || end > len || start > end {
                return Err(RuntimeError {
                    message: format!(
                        "slice indices out of bounds: start={}, end={}, len={}",
                        start, end, len
                    ),
                    stack_trace: vec![],
                });
            }

            let result: String = chars[start as usize..end as usize].iter().collect();
            Ok(Value::String(Rc::new(RefCell::new(result))))
        }
        Value::List(l) => {
            let list = l.borrow();
            let len = list.len() as i32;

            let start = normalize_index(start_idx, len);
            let end = normalize_index(end_idx, len);

            if start < 0 || end < 0 || start > len || end > len || start > end {
                return Err(RuntimeError {
                    message: format!(
                        "slice indices out of bounds: start={}, end={}, len={}",
                        start, end, len
                    ),
                    stack_trace: vec![],
                });
            }

            let result: Vec<Value> = list[start as usize..end as usize].to_vec();
            Ok(Value::List(Rc::new(RefCell::new(result))))
        }
        _ => Err(RuntimeError {
            message: "slice expects a string or list as first argument".to_string(),
            stack_trace: vec![],
        }),
    }
}

fn normalize_index(idx: i32, len: i32) -> i32 {
    if idx < 0 {
        (len + idx).max(0)
    } else {
        idx.min(len)
    }
}
