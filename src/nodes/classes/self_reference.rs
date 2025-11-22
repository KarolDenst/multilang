use crate::error::RuntimeError;
use crate::grammar::Rule;
use crate::node::ParsedChildren;
use crate::node::{Context, Node, Value};

#[derive(Clone)]
pub struct SelfReference {}

impl Node for SelfReference {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        ctx.variables
            .get("this")
            .cloned()
            .ok_or_else(|| RuntimeError {
                message: "'this' used outside of method context".to_string(),
                stack_trace: vec![],
            })
    }

    fn from_children(_rule: Rule, _children: ParsedChildren) -> Box<dyn Node> {
        Box::new(SelfReference {})
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
