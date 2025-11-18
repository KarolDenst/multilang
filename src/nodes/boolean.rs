use crate::node::{Context, Node, Value};

pub struct Boolean {
    pub value: bool,
}

impl Node for Boolean {
    fn run(&self, _ctx: &mut Context) -> Value {
        Value::Bool(self.value)
    }
}
