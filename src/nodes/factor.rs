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
                        Value::Void // Divide by zero
                    } else {
                        Value::Int(l / r)
                    }
                },
            },
            _ => Value::Void,
        }
    }
}
