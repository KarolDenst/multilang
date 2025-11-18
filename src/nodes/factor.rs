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
            _ => Value::Void,
        }
    }

    fn from_children(rule_name: &str, mut children: crate::node::ParsedChildren) -> Box<dyn Node> {
        let left = children.take_child("left").unwrap();
        let right = children.take_child("right").unwrap();
        let op = match rule_name {
            "Mul" => MulOp::Mul,
            "Div" => MulOp::Div,
            _ => panic!("Unknown rule for Factor: {}", rule_name),
        };
        Box::new(Factor { op, left, right })
    }
}
