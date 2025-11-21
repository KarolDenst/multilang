use crate::error::RuntimeError;
use crate::node::{Context, Node, ParsedChildren, Value};

#[derive(Clone)]
pub struct Assignment {
    pub variable_name: String,
    pub expr: Box<dyn Node>,
}

impl Node for Assignment {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        let value = self.expr.run(ctx)?;
        ctx.variables.insert(self.variable_name.clone(), value);
        Ok(Value::Void)
    }

    fn from_children(rule_name: &str, mut children: ParsedChildren) -> Box<dyn Node>
    where
        Self: Sized,
    {
        if rule_name != "Assignment" {
            panic!("Assignment::from_children called with rule {}", rule_name);
        }

        let variable_node = children
            .take_child("name")
            .expect("Assignment missing name");
        let variable_name = variable_node.text().expect("Variable node missing text");

        let expr = children
            .take_child("value")
            .expect("Assignment missing value");

        Box::new(Assignment {
            variable_name,
            expr,
        })
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
