use crate::node::{Context, Node, Value};

pub struct Variable {
    pub name: String,
}

impl Node for Variable {
    fn run(&self, ctx: &mut Context) -> Value {
        if let Some(val) = ctx.variables.get(&self.name) {
            val.clone()
        } else {
            println!("Runtime Error: Variable '{}' not found", self.name);
            Value::Void
        }
    }
    
    fn text(&self) -> Option<String> {
        Some(self.name.clone())
    }
}
