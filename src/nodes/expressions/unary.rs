use crate::error::RuntimeError;
use crate::grammar::Rule;
use crate::node::ParsedChildren;
use crate::node::{Context, Node, Value};

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Not,
    Neg,
}

pub struct Unary {
    pub op: UnaryOp,
    pub expr: Box<dyn Node>,
}

impl Node for Unary {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        let val = self.expr.run(ctx)?;
        match self.op {
            UnaryOp::Not => {
                if let Value::Bool(b) = val {
                    Ok(Value::Bool(!b))
                } else {
                    Err(RuntimeError {
                        message: format!("Expected boolean for unary NOT, got {:?}", val),
                        stack_trace: vec![],
                    })
                }
            }
            UnaryOp::Neg => match val {
                Value::Int(i) => Ok(Value::Int(-i)),
                Value::Float(f) => Ok(Value::Float(-f)),
                _ => Err(RuntimeError {
                    message: format!("Expected number for unary negation, got {:?}", val),
                    stack_trace: vec![],
                }),
            },
        }
    }

    fn from_children(_rule: Rule, mut children: ParsedChildren) -> Box<dyn Node> {
        // Unary = UnaryOp Unary | Atom
        // UnaryOp = [!] | [-]

        // If we have 2 children, it's UnaryOp Unary
        if children.children.len() == 2 {
            let op_node = children.take_child("").unwrap();
            let expr = children.take_child("").unwrap();

            let op_text = op_node.text().unwrap();
            let op = match op_text.as_str() {
                "!" => UnaryOp::Not,
                "-" => UnaryOp::Neg,
                _ => panic!("Unknown UnaryOp: {}", op_text),
            };

            Box::new(Unary { op, expr })
        } else {
            // Just Atom
            children.take_child("").unwrap()
        }
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(Unary {
            op: self.op, // UnaryOp is Copy
            expr: self.expr.clone(),
        })
    }
}
