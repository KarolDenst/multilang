use crate::node::{Context, Node, Value};

#[derive(Debug, Clone, Copy)]
pub enum AddOp {
    Add,
    Sub,
}

pub struct Term {
    pub op: AddOp,
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
}

impl Node for Term {
    fn run(&self, ctx: &mut Context) -> Value {
        let left_val = self.left.run(ctx);
        let right_val = self.right.run(ctx);

        match (left_val, right_val) {
            (Value::Int(l), Value::Int(r)) => match self.op {
                AddOp::Add => Value::Int(l + r),
                AddOp::Sub => Value::Int(l - r),
            },
            (Value::Float(l), Value::Float(r)) => match self.op {
                AddOp::Add => Value::Float(l + r),
                AddOp::Sub => Value::Float(l - r),
            },
            (Value::String(l), Value::String(r)) => match self.op {
                AddOp::Add => Value::String(l + &r),
                AddOp::Sub => Value::Void, // Subtraction not supported for strings
            },
            _ => Value::Void,
        }
    }

    fn from_children(_rule_name: &str, mut children: crate::node::ParsedChildren) -> Box<dyn Node> {
        // Term = Factor AddOp Term | Factor
        let left = children.take_child("").unwrap(); // Factor
        
        if let Some(op_node) = children.take_child("") { // AddOp
            let right = children.take_child("").unwrap(); // Term
            let op_text = op_node.text().unwrap();
            let op = match op_text.as_str() {
                "+" => AddOp::Add,
                "-" => AddOp::Sub,
                _ => panic!("Unknown AddOp: {}", op_text),
            };
            Box::new(Term { op, left, right })
        } else {
            left
        }
    }
}
