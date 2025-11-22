use crate::error::RuntimeError;
use crate::node::{Context, Node, Value};

pub struct Block {
    pub statements: Vec<Box<dyn Node>>,
}

use crate::grammar::Rule;
use crate::node::ParsedChildren;

impl Node for Block {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        let mut last_value = Value::Void;
        for stmt in &self.statements {
            last_value = stmt.run(ctx)?;
        }
        Ok(last_value)
    }

    fn from_children(_rule: Rule, children: ParsedChildren) -> Box<dyn Node> {
        let statements = children
            .remaining()
            .into_iter()
            .map(|(_, node)| node)
            .collect();
        Box::new(Block { statements })
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(Block {
            statements: self.statements.iter().map(|s| s.box_clone()).collect(),
        })
    }
}
