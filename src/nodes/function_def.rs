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
}
