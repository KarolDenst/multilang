use crate::node::{Context, Node, Value};

#[derive(Debug, Clone, Copy)]
pub enum MulOp {
    Mul,
    Div,
}

pub struct Factor {
    pub op: MulOp,
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
}

impl Node for Factor {
    fn run(&self, ctx: &mut Context) -> Value {
        let left_val = self.left.run(ctx);
        let right_val = self.right.run(ctx);

        match (left_val, right_val) {
            (Value::Int(l), Value::Int(r)) => match self.op {
                MulOp::Mul => Value::Int(l * r),
                MulOp::Div => {
                    if r == 0 {
                        Value::Void // TODO: Throw error in case of div by 0
                    } else {
                        Value::Int(l / r)
                    }
                },
            },
            (Value::Float(l), Value::Float(r)) => match self.op {
                MulOp::Mul => Value::Float(l * r),
                MulOp::Div => Value::Float(l / r),
            },
            _ => Value::Void,
        }
    }

    fn from_children(_rule_name: &str, mut children: crate::node::ParsedChildren) -> Box<dyn Node> {
        // Factor = Atom MulOp Factor | Atom
        let left = children.take_child("").unwrap(); // Atom
        
        if let Some(op_node) = children.take_child("") { // MulOp
            let right = children.take_child("").unwrap(); // Factor
            let op_text = op_node.text().unwrap();
            let op = match op_text.as_str() {
                "*" => MulOp::Mul,
                "/" => MulOp::Div,
                _ => panic!("Unknown MulOp: {}", op_text),
            };
            Box::new(Factor { op, left, right })
        } else {
            left
        }
    }
}
