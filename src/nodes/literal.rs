use crate::node::{Context, Node, ParsedChildren, Value};

pub struct Literal {
    pub value: Value,
}

impl Node for Literal {
    fn run(&self, _ctx: &mut Context) -> Value {
        self.value.clone()
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
                Value::String(content)
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
