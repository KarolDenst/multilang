use crate::error::RuntimeError;
use crate::grammar::Rule;
use crate::node::ParsedChildren;
use crate::node::{Context, Node, Value};
use crate::nodes::classes::{MemberAccess, MethodCall};

#[derive(Clone)]
pub struct PostfixNode {
    pub root: Box<dyn Node>,
}

impl Node for PostfixNode {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        self.root.run(ctx)
    }

    fn from_children(_rule: Rule, children: ParsedChildren) -> Box<dyn Node> {
        // Postfix = Atom PostfixSuffix*
        // Children: Atom, PostfixSuffixNode, PostfixSuffixNode...

        let mut iter = children.remaining().into_iter();

        // First is Atom
        let (_, mut current_node) = iter.next().expect("Postfix must have at least Atom");

        for (_, child) in iter {
            if let Some(suffix) = child
                .as_any()
                .downcast_ref::<crate::nodes::classes::postfix_suffix::PostfixSuffixNode>(
            ) {
                match &suffix.suffix_type {
                    crate::nodes::classes::postfix_suffix::SuffixType::Member(member) => {
                        current_node = Box::new(MemberAccess {
                            object: current_node,
                            member: member.clone(),
                        });
                    }
                    crate::nodes::classes::postfix_suffix::SuffixType::Method(method, args) => {
                        current_node = Box::new(MethodCall {
                            object: current_node,
                            method_name: method.clone(),
                            args: args.iter().map(|a| a.box_clone()).collect(),
                        });
                    }
                }
            } else {
                // Should not happen if grammar is correct
                panic!("Unexpected child in PostfixNode");
            }
        }

        Box::new(PostfixNode { root: current_node })
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
