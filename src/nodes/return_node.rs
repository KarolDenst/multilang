use crate::error::RuntimeError;
use crate::node::{Context, Node, Value};

pub struct Return {
    pub expression: Box<dyn Node>,
}

impl Node for Return {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        self.expression.run(ctx)
    }

    fn from_children(_rule_name: &str, mut children: crate::node::ParsedChildren) -> Box<dyn Node> {
        let expr = children.take_child("expression").unwrap();
        Box::new(Return { expression: expr })
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(Return {
            expression: self.expression.clone(),
        })
    }
}
