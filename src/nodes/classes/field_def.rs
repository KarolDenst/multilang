use crate::error::RuntimeError;
use crate::grammar::Rule;
use crate::node::ParsedChildren;
use crate::node::{Context, Node, Value};

#[derive(Clone)]
pub struct FieldDef {
    pub name: String,
}

impl Node for FieldDef {
    fn run(&self, _ctx: &mut Context) -> Result<Value, RuntimeError> {
        Ok(Value::Void)
    }

    fn from_children(_rule: Rule, mut children: ParsedChildren) -> Box<dyn Node> {
        let name_node = children.take_child("name").unwrap();
        let name = name_node.text().unwrap_or_default();
        Box::new(FieldDef { name })
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
