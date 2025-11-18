use crate::node::{Context, Node, Value};

pub struct Return {
    pub expression: Box<dyn Node>,
}

impl Node for Return {
    fn run(&self, ctx: &mut Context) -> Value {
        self.expression.run(ctx)
    }
}
