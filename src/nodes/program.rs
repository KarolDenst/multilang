use crate::error::RuntimeError;
use crate::node::{Context, Node, Value};

pub struct Program {
    pub children: Vec<Box<dyn Node>>,
}

impl Node for Program {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        let mut last_val = Value::Void;
        for stmt in &self.children {
            last_val = stmt.run(ctx)?;
        }
        Ok(last_val)
    }

    fn from_children(
        _rule_name: &str,
        parsed_children: crate::node::ParsedChildren,
    ) -> Box<dyn Node> {
        let children = parsed_children
            .remaining()
            .into_iter()
            .map(|(_, node)| node)
            .collect();
        Box::new(Program { children })
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(Program {
            children: self.children.iter().map(|c| c.box_clone()).collect(),
        })
    }
}
