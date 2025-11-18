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

    fn from_children(rule_name: &str, children: crate::node::ParsedChildren) -> Box<dyn Node> {
        if rule_name == "ParamList" {
             let mut params = Vec::new();
             for item in children.remaining() {
                 let (_, node) = item;
                 if let Some(text) = node.text() {
                     if text != "," {
                         params.push(text);
                     }
                 } else if let Some(sub_params) = node.params() {
                     params.extend(sub_params);
                 }
             }
             Box::new(ListNode { params: Some(params), args: None })
        } else if rule_name == "ArgList" {
             let mut args = Vec::new();
             for item in children.remaining() {
                 let (_, node) = item;
                 if let Some(t) = node.text() {
                     if t != "," {
                         args.push(node);
                     }
                 } else if node.is_args() {
                     args.extend(node.into_args());
                 } else {
                     args.push(node);
                 }
             }
             Box::new(ListNode { params: None, args: Some(args) })
        } else {
            panic!("Unknown rule for ListNode: {}", rule_name);
        }
    }
}
