use crate::node::{Context, Node, Value};

pub struct ListNode {
    pub params: Option<Vec<String>>,
    pub args: Option<Vec<Box<dyn Node>>>,
}

impl Node for ListNode {
    fn run(&self, _ctx: &mut Context) -> Value {
        Value::Void
    }
    
    fn params(&self) -> Option<Vec<String>> {
        self.params.clone()
    }
    
    fn is_args(&self) -> bool {
        self.args.is_some()
    }

    fn into_args(self: Box<Self>) -> Vec<Box<dyn Node>> {
        self.args.unwrap_or_default()
    }
}
