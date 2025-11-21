use crate::error::RuntimeError;
use crate::node::{Context, Node, Value};

#[derive(Debug, Clone, Copy)]
pub enum LogOp {
    And,
    Or,
}

pub struct Logical {
    pub op: LogOp,
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
}

impl Node for Logical {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        let left_val = self.left.run(ctx)?;

        match self.op {
            LogOp::And => {
                if let Value::Bool(b) = left_val {
                    if !b {
                        return Ok(Value::Bool(false));
                    }
                } else {
                    return Err(RuntimeError {
                        message: format!("Expected boolean for logical AND, got {:?}", left_val),
                        stack_trace: vec![],
                    });
                }

                let right_val = self.right.run(ctx)?;
                if let Value::Bool(b) = right_val {
                    Ok(Value::Bool(b))
                } else {
                    Err(RuntimeError {
                        message: format!("Expected boolean for logical AND, got {:?}", right_val),
                        stack_trace: vec![],
                    })
                }
            }
            LogOp::Or => {
                if let Value::Bool(b) = left_val {
                    if b {
                        return Ok(Value::Bool(true));
                    }
                } else {
                    return Err(RuntimeError {
                        message: format!("Expected boolean for logical OR, got {:?}", left_val),
                        stack_trace: vec![],
                    });
                }

                let right_val = self.right.run(ctx)?;
                if let Value::Bool(b) = right_val {
                    Ok(Value::Bool(b))
                } else {
                    Err(RuntimeError {
                        message: format!("Expected boolean for logical OR, got {:?}", right_val),
                        stack_trace: vec![],
                    })
                }
            }
        }
    }

    fn from_children(rule_name: &str, mut children: crate::node::ParsedChildren) -> Box<dyn Node> {
        // LogicalOr = LogicalAnd "||" LogicalOr | LogicalAnd
        // LogicalAnd = Comparison "&&" LogicalAnd | Comparison

        let left = children.take_child("").unwrap();

        // Check if we have a second child (the recursive part)
        if let Some(right) = children.take_child("") {
            let op = match rule_name {
                "LogicalAnd" => LogOp::And,
                "LogicalOr" => LogOp::Or,
                _ => panic!("Unknown logical rule: {}", rule_name),
            };
            Box::new(Logical { op, left, right })
        } else {
            left
        }
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(Logical {
            op: self.op, // LogOp is Copy
            left: self.left.clone(),
            right: self.right.clone(),
        })
    }
}
