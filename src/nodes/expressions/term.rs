use crate::error::RuntimeError;
use crate::grammar::Rule;
use crate::node::ParsedChildren;
use crate::node::{Context, Node, Value};
use std::cell::RefCell;
use std::rc::Rc;

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
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        let left_val = self.left.run(ctx)?;
        let right_val = self.right.run(ctx)?;

        match (left_val, right_val) {
            (Value::Int(l), Value::Int(r)) => match self.op {
                AddOp::Add => Ok(Value::Int(l + r)),
                AddOp::Sub => Ok(Value::Int(l - r)),
            },
            (Value::Float(l), Value::Float(r)) => match self.op {
                AddOp::Add => Ok(Value::Float(l + r)),
                AddOp::Sub => Ok(Value::Float(l - r)),
            },
            (Value::String(l), Value::String(r)) => match self.op {
                AddOp::Add => Ok(Value::String(Rc::new(RefCell::new(
                    l.borrow().clone() + &r.borrow(),
                )))),
                AddOp::Sub => Err(RuntimeError {
                    message: "Subtraction not supported for strings".to_string(),
                    stack_trace: vec![],
                }),
            },
            (l, r) => Err(RuntimeError {
                message: format!(
                    "Invalid operands for addition/subtraction: {:?} and {:?}",
                    l, r
                ),
                stack_trace: vec![],
            }),
        }
    }

    fn from_children(_rule: Rule, mut children: ParsedChildren) -> Box<dyn Node> {
        // Term = Factor AddOp Term | Factor
        let left = children.take_child("").unwrap();

        if let Some(op_node) = children.take_child("") {
            let right = children.take_child("").unwrap();

            let op = match op_node.rule() {
                Some(Rule::Add) => AddOp::Add,
                Some(Rule::Sub) => AddOp::Sub,
                _ => panic!("Unknown AddOp rule: {:?}", op_node.rule()),
            };
            Box::new(Term { op, left, right })
        } else {
            left
        }
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(Term {
            op: self.op,
            left: self.left.clone(),
            right: self.right.clone(),
        })
    }
}
