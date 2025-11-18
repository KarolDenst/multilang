use crate::node::{Context, Node, Value};

pub struct Int {
    pub value: i32,
}

impl Node for Int {
    fn run(&self, _ctx: &mut Context) -> Value {
        Value::Int(self.value)
    }
}
