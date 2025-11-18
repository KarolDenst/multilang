use crate::node::{Context, Node, Value};

pub struct If {
    pub condition: Box<dyn Node>,
    pub then_block: Box<dyn Node>,
    pub else_block: Option<Box<dyn Node>>,
}

impl Node for If {
    fn run(&self, ctx: &mut Context) -> Value {
        let condition_val = self.condition.run(ctx);
        
        let is_true = match condition_val {
            Value::Bool(b) => b,
            Value::Int(i) => i != 0,
            _ => false,
        };

        if is_true {
            self.then_block.run(ctx)
        } else if let Some(else_block) = &self.else_block {
            else_block.run(ctx)
        } else {
            Value::Void
        }
    }
}
