use crate::error::RuntimeError;
use crate::grammar::Rule;
use crate::node::ParsedChildren;
use crate::node::{Context, Node, Value};

pub struct ForNode {
    pub variable_name: String,
    pub iterable: Box<dyn Node>,
    pub body: Box<dyn Node>,
}

impl Node for ForNode {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        let iterable_val = self.iterable.run(ctx)?;

        if let Value::List(list_rc) = iterable_val {
            // We need to clone the list to iterate safely without holding a borrow across body execution
            // because body execution might modify the list (though that would be weird in a for loop, but possible via append)
            // Actually, if we modify the list while iterating, it might be tricky.
            // Standard behavior: iterate over the snapshot or live?
            // Let's iterate over a snapshot (clone the vector).
            let elements = list_rc.borrow().clone();

            for element in elements {
                ctx.variables.insert(self.variable_name.clone(), element);
                self.body.run(ctx)?;
            }
            Ok(Value::Void)
        } else {
            Err(RuntimeError {
                message: format!("For loop expects a list, got {:?}", iterable_val),
                stack_trace: vec![],
            })
        }
    }

    fn from_children(_rule: Rule, mut children: ParsedChildren) -> Box<dyn Node> {
        // ForLoop = "for" variable:Identifier "in" iterable:Expr "{" body:Block "}"

        let variable_node = children.take_child("variable").unwrap();
        let variable_name = variable_node
            .text()
            .expect("For loop variable must be an identifier");

        let iterable = children.take_child("iterable").unwrap();
        let body = children.take_child("body").unwrap();

        Box::new(ForNode {
            variable_name,
            iterable,
            body,
        })
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(ForNode {
            variable_name: self.variable_name.clone(),
            iterable: self.iterable.clone(),
            body: self.body.clone(),
        })
    }
}
