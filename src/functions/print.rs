use crate::error::RuntimeError;
use crate::node::Value;

pub fn print_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        match arg {
            Value::Int(val) => print!("{}", val),
            Value::Float(val) => print!("{}", val),
            Value::String(val) => print!("{}", val),
            Value::Bool(val) => print!("{}", val),
            Value::List(l) => {
                let list = l.borrow();
                print!("[");
                for (i, val) in list.iter().enumerate() {
                    if i > 0 {
                        print!(", ");
                    }
                    match val {
                        Value::Int(v) => print!("{}", v),
                        Value::Float(v) => print!("{}", v),
                        Value::String(v) => print!("\"{}\"", v),
                        Value::Bool(v) => print!("{}", v),
                        Value::List(_) => print!("[...]"), // Nested list simplified
                        Value::Void => print!("void"),
                    }
                }
                print!("]");
            }
            Value::Void => print!("(void)"),
        }
    }
    println!();
    Ok(Value::Void)
}
