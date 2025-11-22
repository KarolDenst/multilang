use crate::error::RuntimeError;
use crate::node::{Context, Node, Value};

pub struct ArgListNode {
    pub params: Option<Vec<String>>,
    pub args: Option<Vec<Box<dyn Node>>>,
}

use crate::grammar::Rule;
use crate::node::ParsedChildren;

impl Node for ArgListNode {
    fn run(&self, _ctx: &mut Context) -> Result<Value, RuntimeError> {
        // ArgList doesn't run directly, it's used by FunctionCall
        Ok(Value::Void)
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

    fn from_children(rule: Rule, children: ParsedChildren) -> Box<dyn Node> {
        if rule == Rule::ParamList {
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
            Box::new(ArgListNode {
                params: Some(params),
                args: None,
            })
        } else if rule == Rule::ArgList {
            let mut args = Vec::new();
            for item in children.remaining() {
                let (_, node) = item;
                if let Some(t) = node.text()
                    && t == "," {
                        continue;
                    }

                if node.is_args() {
                    args.extend(node.into_args());
                } else {
                    args.push(node);
                }
            }
            Box::new(ArgListNode {
                params: None,
                args: Some(args),
            })
        } else {
            panic!("Unknown rule for ArgListNode: {:?}", rule);
        }
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(ArgListNode {
            params: self.params.clone(),
            args: self
                .args
                .as_ref()
                .map(|args| args.iter().map(|a| a.box_clone()).collect()),
        })
    }
}
