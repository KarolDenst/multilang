use crate::error::RuntimeError;
use crate::grammar::Rule;
use crate::node::ParsedChildren;
use crate::node::{Context, Node, Value};

pub struct If {
    pub condition: Box<dyn Node>,
    pub then_block: Box<dyn Node>,
    pub else_block: Option<Box<dyn Node>>,
}

impl Node for If {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        let condition_val = self.condition.run(ctx)?;

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
            Ok(Value::Void)
        }
    }

    fn from_children(_rule: Rule, mut children: ParsedChildren) -> Box<dyn Node> {
        let condition = children.take_child("condition").unwrap();
        let then_block = children.take_child("then").unwrap();
        let else_block = children.take_child("else");
        Box::new(If {
            condition,
            then_block,
            else_block,
        })
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(If {
            condition: self.condition.clone(),
            then_block: self.then_block.clone(),
            else_block: self.else_block.clone(),
        })
    }
}
