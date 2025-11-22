use crate::error::RuntimeError;
use crate::grammar::Rule;
use crate::node::ParsedChildren;
use crate::node::{Context, Node, Value};

pub struct Variable {
    pub name: String,
}

impl Node for Variable {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        if let Some(val) = ctx.variables.get(&self.name) {
            Ok(val.clone())
        } else {
            Err(RuntimeError {
                message: format!("Variable '{}' not found", self.name),
                stack_trace: vec![],
            })
        }
    }

    fn text(&self) -> Option<String> {
        Some(self.name.clone())
    }

    fn from_children(_rule: Rule, children: ParsedChildren) -> Box<dyn Node> {
        let child = children
            .remaining()
            .into_iter()
            .next()
            .map(|(_, node)| node)
            .unwrap();
        let name = child.text().unwrap_or_default();
        Box::new(Variable { name })
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(Variable {
            name: self.name.clone(),
        })
    }
}
