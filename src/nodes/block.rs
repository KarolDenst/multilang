use crate::node::{Context, Node, Value};

pub struct Block {
    pub statements: Vec<Box<dyn Node>>,
}

impl Node for Block {
    fn run(&self, ctx: &mut Context) -> Value {
        let mut last_val = Value::Void;
        for stmt in &self.statements {
            last_val = stmt.run(ctx);
            // If we implement return properly, we might need to check for it here.
            // But for now, let's just run all statements.
            // Wait, if a statement is a Return, we should probably stop?
            // The current Return node just returns a value, but doesn't signal control flow.
            // The Program node also just runs everything.
            // For now, I'll keep it simple and just run everything, returning the last value.
            // Real control flow requires more complex return handling (e.g. Result<Value, ReturnValue>).
            // Given the current architecture, I'll stick to simple execution.
        }
        last_val
    }

    fn from_children(_rule_name: &str, children: crate::node::ParsedChildren) -> Box<dyn Node> {
        let stmts = children.remaining().into_iter().map(|(_, node)| node).collect();
        Box::new(Block { statements: stmts })
    }
}
