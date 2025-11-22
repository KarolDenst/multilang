use crate::error::RuntimeError;
use crate::grammar::Rule;
use crate::node::ParsedChildren;
use crate::node::{Context, Node, Value};
use std::rc::Rc;

#[derive(Clone)]
pub struct MethodDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: Rc<dyn Node>,
}

impl Node for MethodDef {
    fn run(&self, _ctx: &mut Context) -> Result<Value, RuntimeError> {
        Ok(Value::Void)
    }

    fn from_children(_rule: Rule, mut children: ParsedChildren) -> Box<dyn Node> {
        let name_node = children.take_child("name").unwrap();
        let name = name_node.text().unwrap_or_default();

        let mut params = Vec::new();
        if let Some(param_list) = children.take_child("params")
            && let Some(p_list) = param_list.params()
        {
            params = p_list;
        }

        let body = children.take_child("body").unwrap();
        Box::new(MethodDef {
            name,
            params,
            body: Rc::from(body),
        })
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
