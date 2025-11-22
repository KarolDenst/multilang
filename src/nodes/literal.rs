use crate::error::RuntimeError;
use crate::node::{Context, Node, ParsedChildren, Value};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Literal {
    pub value: Value,
}

impl Node for Literal {
    fn run(&self, _ctx: &mut Context) -> Result<Value, RuntimeError> {
        Ok(self.value.clone())
    }

    fn text(&self) -> Option<String> {
        match &self.value {
            Value::Int(v) => Some(v.to_string()),
            Value::Float(v) => Some(v.to_string()),
            Value::String(v) => Some(format!("\"{}\"", v.borrow())), // Re-add quotes to match original token if possible, or just return content?
            // The parser expects the original token text including quotes for String literals in MapEntryNode logic.
            // However, we stripped quotes in from_children.
            // MapEntryNode expects quotes if it was a String literal.
            // Let's return the content with quotes for String.
            Value::Bool(v) => Some(v.to_string()),
            _ => None,
        }
    }

    fn from_children(rule_name: &str, mut children: ParsedChildren) -> Box<dyn Node> {
        let child = children
            .take_child("")
            .or_else(|| children.remaining().into_iter().next().map(|(_, n)| n));

        let value = match rule_name {
            "Int" => {
                let text = child.unwrap().text().unwrap_or_default();
                Value::Int(text.parse().unwrap_or(0))
            }
            "Float" => {
                let text = child.unwrap().text().unwrap_or_default();
                Value::Float(text.parse().unwrap_or(0.0))
            }
            "String" => {
                let text = child.unwrap().text().unwrap_or_default();
                // Remove quotes
                let content = if text.len() >= 2 {
                    text[1..text.len() - 1].to_string()
                } else {
                    String::new()
                };
                Value::String(Rc::new(RefCell::new(content)))
            }
            "True" => Value::Bool(true),
            "False" => Value::Bool(false),
            _ => panic!("Unknown rule for Literal: {}", rule_name),
        };

        Box::new(Literal { value })
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(Literal {
            value: self.value.clone(),
        })
    }
}
