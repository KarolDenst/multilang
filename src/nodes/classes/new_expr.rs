use crate::error::RuntimeError;
use crate::grammar::Rule;
use crate::node::ParsedChildren;
use crate::node::{Context, Node, Object, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct NewExpr {
    pub class_name: String,
    pub args: Vec<Box<dyn Node>>,
}

impl Node for NewExpr {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        // 1. Look up class
        let class = ctx
            .classes
            .get(&self.class_name)
            .ok_or_else(|| RuntimeError {
                message: format!("Undefined class '{}'", self.class_name),
                stack_trace: vec![],
            })?
            .clone();

        // 2. Check arg count
        if self.args.len() != class.fields.len() {
            return Err(RuntimeError {
                message: format!(
                    "Class '{}' expects {} arguments, got {}",
                    self.class_name,
                    class.fields.len(),
                    self.args.len()
                ),
                stack_trace: vec![],
            });
        }

        // 3. Evaluate args and populate fields
        let mut fields = HashMap::new();
        for (i, arg_node) in self.args.iter().enumerate() {
            let val = arg_node.run(ctx)?;
            fields.insert(class.fields[i].clone(), val);
        }

        // 4. Create object
        Ok(Value::Object(Rc::new(RefCell::new(Object {
            class_name: self.class_name.clone(),
            fields,
        }))))
    }

    fn from_children(_rule: Rule, mut children: ParsedChildren) -> Box<dyn Node> {
        let name_node = children.take_child("class_name").unwrap();
        let class_name = name_node.text().unwrap_or_default();

        let mut args = Vec::new();
        if let Some(arg_list) = children.take_child("args") {
            args = arg_list.into_args();
        }

        Box::new(NewExpr { class_name, args })
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
