use crate::error::RuntimeError;
use crate::grammar::Rule;
use crate::node::ParsedChildren;
use crate::node::{Context, Node, Value};

#[derive(Debug, Clone, Copy)]
pub enum MulOp {
    Mul,
    Div,
    Mod,
}

pub struct Factor {
    pub op: MulOp,
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
}

impl Node for Factor {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        let left_val = self.left.run(ctx)?;
        let right_val = self.right.run(ctx)?;

        match (left_val, right_val) {
            (Value::Int(l), Value::Int(r)) => match self.op {
                MulOp::Mul => Ok(Value::Int(l * r)),
                MulOp::Div => {
                    if r == 0 {
                        Err(RuntimeError {
                            message: "Division by zero".to_string(),
                            stack_trace: vec![],
                        })
                    } else {
                        Ok(Value::Int(l / r))
                    }
                }
                MulOp::Mod => {
                    if r == 0 {
                        Err(RuntimeError {
                            message: "Modulo by zero".to_string(),
                            stack_trace: vec![],
                        })
                    } else {
                        Ok(Value::Int(l % r))
                    }
                }
            },
            (Value::Float(l), Value::Float(r)) => match self.op {
                MulOp::Mul => Ok(Value::Float(l * r)),
                MulOp::Div => Ok(Value::Float(l / r)),
                MulOp::Mod => Ok(Value::Float(l % r)),
            },
            (l, r) => Err(RuntimeError {
                message: format!(
                    "Invalid operands for multiplication/division/modulo: {:?} and {:?}",
                    l, r
                ),
                stack_trace: vec![],
            }),
        }
    }

    fn from_children(_rule: Rule, mut children: ParsedChildren) -> Box<dyn Node> {
        // Factor = Atom MulOp Factor | Atom
        let left = children.take_child("").unwrap();

        if let Some(op_node) = children.take_child("") {
            let right = children.take_child("").unwrap();
            let op = match op_node.rule() {
                Some(Rule::Mul) => MulOp::Mul,
                Some(Rule::Div) => MulOp::Div,
                Some(Rule::Mod) => MulOp::Mod,
                _ => panic!("Unknown MulOp rule: {:?}", op_node.rule()),
            };
            Box::new(Factor { op, left, right })
        } else {
            left
        }
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(Factor {
            op: self.op,
            left: self.left.clone(),
            right: self.right.clone(),
        })
    }
}
