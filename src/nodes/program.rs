use crate::error::RuntimeError;
use crate::grammar::Rule;
use crate::node::ParsedChildren;
use crate::node::{Context, Node, Value};

pub struct Program {
    pub children: Vec<Box<dyn Node>>,
}

impl Node for Program {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        let mut last_value = Value::Void;
        for stmt in &self.children {
            // Changed from self.statements to self.children to match struct definition
            last_value = stmt.run(ctx)?;
        }
        Ok(last_value)
    }

    fn from_children(_rule: Rule, parsed_children: ParsedChildren) -> Box<dyn Node> {
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
