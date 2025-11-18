use crate::node::{Context, Node, Value};

pub struct Int {
    pub value: i32,
}

impl Node for Int {
    fn run(&self, _ctx: &mut Context) -> Value {
        Value::Int(self.value)
    }

    fn from_children(_rule_name: &str, mut children: crate::node::ParsedChildren) -> Box<dyn Node> {
        let child = children.take_child("").or_else(|| children.remaining().into_iter().next().map(|(_, n)| n)).unwrap();
        let text = child.text().unwrap_or_default();
        Box::new(Int { value: text.parse().unwrap_or(0) })
    }
}
