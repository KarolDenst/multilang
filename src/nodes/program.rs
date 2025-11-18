use crate::node::{Context, Node, Value};

pub struct Program {
    pub statements: Vec<Box<dyn Node>>,
}

impl Node for Program {
    fn run(&self, ctx: &mut Context) -> Value {
        let mut last_val = Value::Void;
        for stmt in &self.statements {
            last_val = stmt.run(ctx);
        }
        last_val
    }
}
