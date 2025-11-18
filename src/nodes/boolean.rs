use crate::node::{Context, Node, Value};

pub struct Boolean {
    pub value: bool,
}

impl Node for Boolean {
    fn run(&self, _ctx: &mut Context) -> Value {
        Value::Bool(self.value)
    }

    fn from_children(rule_name: &str, _children: crate::node::ParsedChildren) -> Box<dyn Node> {
        let value = match rule_name {
            "True" => true,
            "False" => false,
            _ => panic!("Unknown rule for Boolean: {}", rule_name),
        };
        Box::new(Boolean { value })
    }
}
