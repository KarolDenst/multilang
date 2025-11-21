use crate::error::RuntimeError;
use crate::node::{Context, Node, Value};

pub struct WhileNode {
    pub condition: Box<dyn Node>,
    pub body: Box<dyn Node>,
}

impl Node for WhileNode {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        loop {
            let condition_val = self.condition.run(ctx)?;
            let is_true = match condition_val {
                Value::Bool(b) => b,
                Value::Int(i) => i != 0,
                _ => false,
            };

            if !is_true {
                break;
            }

            self.body.run(ctx)?;
        }
        Ok(Value::Void)
    }

    fn from_children(_rule_name: &str, mut children: crate::node::ParsedChildren) -> Box<dyn Node> {
        // WhileLoop = "while" condition:Expr "{" body:Block "}"
        // Or similar. The parser usually names children if we use labels in grammar.
        // Assuming grammar: WhileLoop = "while" condition:Expr "{" body:Block "}"

        let condition = children.take_child("condition").unwrap();
        let body = children.take_child("body").unwrap();

        Box::new(WhileNode { condition, body })
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(WhileNode {
            condition: self.condition.clone(),
            body: self.body.clone(),
        })
    }
}
