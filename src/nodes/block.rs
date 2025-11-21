use crate::error::RuntimeError;
use crate::node::{Context, Node, Value};

pub struct Block {
    pub program: Box<dyn Node>,
}

impl Node for Block {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        self.program.run(ctx)
    }

    fn from_children(_rule_name: &str, mut children: crate::node::ParsedChildren) -> Box<dyn Node> {
        let program = children.take_child("").unwrap();
        Box::new(Block { program })
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(Block {
            program: self.program.box_clone(), // Use box_clone on the inner node
        })
    }
}
