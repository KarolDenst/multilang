use crate::node::{Context, Node, Value};

pub struct Print {
    pub expression: Box<dyn Node>,
}

impl Node for Print {
    fn run(&self, ctx: &mut Context) -> Value {
        let val = self.expression.run(ctx);
        match val {
            Value::Int(i) => println!("{}", i),
            Value::Float(f) => println!("{}", f),
            Value::String(s) => println!("{}", s),
            Value::Bool(b) => println!("{}", b),
            Value::Void => println!("(void)"),
        }
        Value::Void
    }

    fn from_children(_rule_name: &str, mut children: crate::node::ParsedChildren) -> Box<dyn Node> {
        let expr = children.take_child("expression").unwrap();
        Box::new(Print { expression: expr })
    }
}
