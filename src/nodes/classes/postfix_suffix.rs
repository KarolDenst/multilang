use crate::error::RuntimeError;
use crate::grammar::Rule;
use crate::node::ParsedChildren;
use crate::node::{Context, Node, Value};

#[derive(Clone)]
pub enum SuffixType {
    Member(String),
    Method(String, Vec<Box<dyn Node>>),
}

#[derive(Clone)]
pub struct PostfixSuffixNode {
    pub suffix_type: SuffixType,
}

impl Node for PostfixSuffixNode {
    fn run(&self, _ctx: &mut Context) -> Result<Value, RuntimeError> {
        Err(RuntimeError {
            message: "PostfixSuffixNode should not be run directly".to_string(),
            stack_trace: vec![],
        })
    }

    fn from_children(_rule: Rule, mut children: ParsedChildren) -> Box<dyn Node> {
        // PostfixSuffix = "." member:Identifier
        // PostfixSuffix = "." method:Identifier "(" args:ArgList ")"
        // PostfixSuffix = "." method:Identifier "(" ")"

        if let Some(member_node) = children.take_child("member") {
            let member = member_node.text().unwrap_or_default();
            return Box::new(PostfixSuffixNode {
                suffix_type: SuffixType::Member(member),
            });
        } else if let Some(method_node) = children.take_child("method") {
            let method = method_node.text().unwrap_or_default();
            let mut args = Vec::new();

            if let Some(args_node) = children.take_child("args") {
                if let Some(arg_list) = args_node
                    .as_any()
                    .downcast_ref::<crate::nodes::functions::ArgListNode>()
                {
                    if let Some(ref args_vec) = arg_list.args {
                        args = args_vec.iter().map(|a| a.box_clone()).collect();
                    }
                }
            }

            return Box::new(PostfixSuffixNode {
                suffix_type: SuffixType::Method(method, args),
            });
        }

        panic!("Invalid PostfixSuffix children");
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
