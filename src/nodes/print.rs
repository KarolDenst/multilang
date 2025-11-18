use crate::node::{Context, Node, Value};

pub struct Print {
    pub expression: Box<dyn Node>,
}

impl Node for Print {
    fn run(&self, ctx: &mut Context) -> Value {
        let val = self.expression.run(ctx);
        match val {
            Value::Int(i) => println!("{}", i),
            Value::Bool(b) => println!("{}", b),
            Value::Void => println!("(void)"),
        }
        Value::Void
    }
}
