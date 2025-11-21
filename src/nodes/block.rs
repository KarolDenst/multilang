use crate::error::RuntimeError;
use crate::node::{Context, Node, Value};

pub struct Block {
    pub statements: Vec<Box<dyn Node>>,
}

impl Node for Block {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        let mut last_value = Value::Void;
        for stmt in &self.statements {
            last_value = stmt.run(ctx)?;
        }
        Ok(last_value)
    }

    fn from_children(
        _rule_name: &str,
        parsed_children: crate::node::ParsedChildren,
    ) -> Box<dyn Node> {
        let statements = parsed_children
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
