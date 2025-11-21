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
            Value::Void => print!("(void)"),
        }
    }
    println!();
    Ok(Value::Void)
}
