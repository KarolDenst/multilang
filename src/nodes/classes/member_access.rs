use crate::error::RuntimeError;
use crate::grammar::Rule;
use crate::node::ParsedChildren;
use crate::node::{Context, Node, Value};

#[derive(Clone)]
pub struct MemberAccess {
    pub object: Box<dyn Node>,
    pub member: String,
}

impl Node for MemberAccess {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        let obj_val = self.object.run(ctx)?;

        if let Value::Object(obj_rc) = obj_val {
            let obj = obj_rc.borrow();
            if let Some(val) = obj.fields.get(&self.member) {
                return Ok(val.clone());
            } else {
                return Err(RuntimeError {
                    message: format!(
                        "Object of class '{}' has no field '{}'",
                        obj.class_name, self.member
                    ),
                    stack_trace: vec![],
                });
            }
        } else {
            return Err(RuntimeError {
                message: format!("Cannot access member '{}' on non-object", self.member),
                stack_trace: vec![],
            });
        }
    }

    fn from_children(_rule: Rule, mut children: ParsedChildren) -> Box<dyn Node> {
        let object = children.take_child("object").unwrap();
        let member_node = children.take_child("member").unwrap();
        let member = member_node.text().unwrap_or_default();

        Box::new(MemberAccess { object, member })
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
