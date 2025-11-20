use crate::node::{Context, Node, Value};

pub enum UnaryOp {
    Not,
}

pub struct Unary {
    pub op: UnaryOp,
    pub expr: Box<dyn Node>,
}

impl Node for Unary {
    fn run(&self, ctx: &mut Context) -> Value {
        let val = self.expr.run(ctx);
        match self.op {
            UnaryOp::Not => {
                if let Value::Bool(b) = val {
                    Value::Bool(!b)
                } else {
                    panic!("Expected boolean for unary NOT");
                }
            }
        }
    }

    fn from_children(_rule_name: &str, mut children: crate::node::ParsedChildren) -> Box<dyn Node> {
        // Unary = UnaryOp Unary | Atom
        // UnaryOp = [!]
        
        // If we have 2 children, it's UnaryOp Unary
        if children.children.len() == 2 {
            let op_node = children.take_child("").unwrap();
            let expr = children.take_child("").unwrap();
            
            let op_text = op_node.text().unwrap();
            let op = match op_text.as_str() {
                "!" => UnaryOp::Not,
                _ => panic!("Unknown UnaryOp: {}", op_text),
            };
            
            Box::new(Unary { op, expr })
        } else {
            // Just Atom
            children.take_child("").unwrap()
        }
    }
}
