use crate::error::RuntimeError;
use crate::node::Value;

pub fn print_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print_value(arg);
    }
    println!();
    Ok(Value::Void)
}

fn print_value(val: &Value) {
    match val {
        Value::Int(v) => print!("{}", v),
        Value::Float(v) => print!("{}", v),
        Value::String(v) => print!("{}", v.borrow()),
        Value::Bool(v) => print!("{}", v),
        Value::List(l) => {
            let list = l.borrow();
            print!("[");
            for (i, item) in list.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print_value(item);
            }
            print!("]");
        }
        Value::Map(m) => {
            let map = m.borrow();
            print!("{{");
            // Sort keys for deterministic output
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            for (i, key) in keys.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("{}: ", key);
                print_value(&map[*key]);
            }
            print!("}}");
        }
        Value::Void => print!("(void)"),
    }
}
