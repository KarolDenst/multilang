use crate::error::RuntimeError;
use crate::node::Value;
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    pub static TEST_LOGS: RefCell<Option<Rc<RefCell<Vec<String>>>>> = RefCell::new(None);
}

fn write_output(s: &str) {
    TEST_LOGS.with(|logs| {
        if let Some(logs) = &*logs.borrow() {
            logs.borrow_mut().push(s.to_string());
        } else {
            print!("{}", s);
        }
    });
}

pub fn print_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
    let mut output = String::new();
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            output.push(' ');
        }
        output.push_str(&format_value(arg));
    }
    output.push('\n');
    write_output(&output);
    Ok(Value::Void)
}

fn format_value(val: &Value) -> String {
    match val {
        Value::Int(v) => format!("{}", v),
        Value::Float(v) => format!("{}", v),
        Value::String(v) => v.borrow().clone(),
        Value::Bool(v) => format!("{}", v),
        Value::List(l) => {
            let list = l.borrow();
            let elements: Vec<String> = list.iter().map(|v| format_value(v)).collect();
            format!("[{}]", elements.join(", "))
        }
        Value::Map(m) => {
            let map = m.borrow();
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            let elements: Vec<String> = keys
                .iter()
                .map(|k| format!("{}: {}", k, format_value(&map[*k])))
                .collect();
            format!("{{{}}}", elements.join(", "))
        }
        Value::Void => "(void)".to_string(),
        Value::Object(obj) => {
            let obj = obj.borrow();
            format!("<Object {}>", obj.class_name)
        }
    }
}
