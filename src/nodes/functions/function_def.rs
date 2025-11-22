use crate::error::RuntimeError;
use crate::grammar::Rule;
use crate::node::ParsedChildren;
use crate::node::{Context, Node, Value};
use std::rc::Rc;

pub struct FunctionDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: Rc<dyn Node>,
    pub line: usize,
}

impl Node for FunctionDef {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        ctx.functions.insert(
            self.name.clone(),
            crate::node::Function {
                params: self.params.clone(),
                body: self.body.clone(),
            },
        );
        Ok(Value::Void)
    }

    fn from_children(_rule: Rule, mut children: ParsedChildren) -> Box<dyn Node> {
        let line = children.line;
        let name_node = children.take_child("name").unwrap();
        let name = name_node.text().unwrap_or_default();

        let mut params = Vec::new();
        if let Some(param_list) = children.take_child("params")
            && let Some(p_list) = param_list.params() {
                params = p_list;
            }

        let body = children.take_child("body").unwrap();
        Box::new(FunctionDef {
            name,
            params,
            body: Rc::from(body),
            line,
        })
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(FunctionDef {
            name: self.name.clone(),
            params: self.params.clone(),
            body: self.body.clone(),
            line: self.line,
        })
    }
}
