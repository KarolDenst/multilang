use crate::node::{Context, Node, Value};
use std::rc::Rc;

pub struct FunctionDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: Rc<dyn Node>,
}

impl Node for FunctionDef {
    fn run(&self, ctx: &mut Context) -> Value {
        ctx.functions.insert(self.name.clone(), crate::node::Function {
            params: self.params.clone(),
            body: self.body.clone(),
        });
        Value::Void
    }

    fn from_children(_rule_name: &str, mut children: crate::node::ParsedChildren) -> Box<dyn Node> {
        let name_node = children.take_child("name").unwrap();
        let name = name_node.text().unwrap_or_default();
        
        let mut params = Vec::new();
        if let Some(param_list) = children.take_child("params") {
            if let Some(p_list) = param_list.params() {
                params = p_list;
            }
        }
        
        let body = children.take_child("body").unwrap();
        Box::new(FunctionDef { name, params, body: Rc::from(body) })
    }
}
