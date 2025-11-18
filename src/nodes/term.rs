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
            _ => Value::Void,
        }
    }

    fn from_children(rule_name: &str, mut children: crate::node::ParsedChildren) -> Box<dyn Node> {
        let left = children.take_child("left").unwrap();
        let right = children.take_child("right").unwrap();
        let op = match rule_name {
            "Add" => AddOp::Add,
            "Sub" => AddOp::Sub,
            _ => panic!("Unknown rule for Term: {}", rule_name),
        };
        Box::new(Term { op, left, right })
    }
}
