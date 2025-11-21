use crate::error::RuntimeError;
use crate::node::{Context, Node, Value};
use std::cell::RefCell;
use std::rc::Rc;

pub struct ListNode {
    pub elements: Vec<Box<dyn Node>>,
}

pub struct ElementsNode {
    pub elements: Vec<Box<dyn Node>>,
}

impl Node for ElementsNode {
    fn run(&self, _ctx: &mut Context) -> Result<Value, RuntimeError> {
        Err(RuntimeError {
            message: "ElementsNode should not be run directly".to_string(),
            stack_trace: vec![],
        })
    }

    fn from_children(rule_name: &str, children: crate::node::ParsedChildren) -> Box<dyn Node> {
        if rule_name == "Elements" {
            let mut elements = Vec::new();
            for item in children.remaining() {
                let (_, node) = item;
                if let Some(t) = node.text() {
                    if t == "," {
                        continue;
                    }
                }

                if node.is_list_elements() {
                    elements.extend(node.into_list_elements());
                } else {
                    elements.push(node);
                }
            }
            Box::new(ElementsNode { elements })
        } else {
            panic!("Unknown rule for ElementsNode: {}", rule_name);
        }
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(ElementsNode {
            elements: self.elements.iter().map(|e| e.box_clone()).collect(),
        })
    }

    fn is_list_elements(&self) -> bool {
        true
    }

    fn into_list_elements(self: Box<Self>) -> Vec<Box<dyn Node>> {
        self.elements
    }
}

impl Node for ListNode {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        let mut values = Vec::new();
        for element in &self.elements {
            values.push(element.run(ctx)?);
        }
        Ok(Value::List(Rc::new(RefCell::new(values))))
    }

    fn from_children(rule_name: &str, children: crate::node::ParsedChildren) -> Box<dyn Node> {
        if rule_name == "ListLiteral" {
            let mut elements = Vec::new();
            for item in children.remaining() {
                let (_, node) = item;
                if node.is_list_elements() {
                    elements = node.into_list_elements();
                    break;
                }
            }
            Box::new(ListNode { elements })
        } else {
            panic!("Unknown rule for ListNode: {}", rule_name);
        }
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(ListNode {
            elements: self.elements.iter().map(|e| e.box_clone()).collect(),
        })
    }

    fn is_list_elements(&self) -> bool {
        false
    }

    fn into_list_elements(self: Box<Self>) -> Vec<Box<dyn Node>> {
        vec![]
    }
}
