use crate::node::{Context, Node, Value};

pub enum CompOp {
    Equal,
    NotEqual,
    Less,
    Greater,
}

pub struct Comparison {
    pub op: CompOp,
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
}

impl Node for Comparison {
    fn run(&self, ctx: &mut Context) -> Value {
        let left_val = self.left.run(ctx);
        let right_val = self.right.run(ctx);

        let result = match (&left_val, &right_val) {
            (Value::Int(l), Value::Int(r)) => match self.op {
                CompOp::Equal => l == r,
                CompOp::NotEqual => l != r,
                CompOp::Less => l < r,
                CompOp::Greater => l > r,
            },
            (Value::Float(l), Value::Float(r)) => match self.op {
                CompOp::Equal => (l - r).abs() < f64::EPSILON,
                CompOp::NotEqual => (l - r).abs() >= f64::EPSILON,
                CompOp::Less => l < r,
                CompOp::Greater => l > r,
            },
            (Value::String(l), Value::String(r)) => match self.op {
                CompOp::Equal => l == r,
                CompOp::NotEqual => l != r,
                CompOp::Less => l < r,
                CompOp::Greater => l > r,
            },
            (Value::Bool(l), Value::Bool(r)) => match self.op {
                CompOp::Equal => l == r,
                CompOp::NotEqual => l != r,
                CompOp::Less => l < r, // False < True
                CompOp::Greater => l > r,
            },
            _ => {
                // Type mismatch or unsupported types, return False (or maybe Void/Error?)
                // For now, let's say they are not equal, and comparison fails for ordering.
                match self.op {
                    CompOp::Equal => false,
                    CompOp::NotEqual => true,
                    _ => false, // Cannot compare order of different types
                }
            }
        };

        Value::Bool(result)
    }

    fn from_children(_rule_name: &str, mut children: crate::node::ParsedChildren) -> Box<dyn Node> {
        // Comparison = Term CompOp Term | Term
        // If it's just Term, it will be returned directly by the parser logic if we set it up right,
        // OR we handle it here.
        // Based on previous Term/Factor implementation:
        // If we have an operator, we create the node. If not, we return the child.
        
        let left = children.take_child("").unwrap(); // Term
        
        if let Some(op_node) = children.take_child("") { // CompOp
            let right = children.take_child("").unwrap(); // Term
            let op_text = op_node.text().unwrap();
            let op = match op_text.as_str() {
                "==" => CompOp::Equal,
                "!=" => CompOp::NotEqual,
                "<" => CompOp::Less,
                ">" => CompOp::Greater,
                _ => panic!("Unknown CompOp: {}", op_text),
            };
            Box::new(Comparison { op, left, right })
        } else {
            left
        }
    }
}
