use crate::error::RuntimeError;
use crate::grammar::Rule;
use crate::node::ParsedChildren;
use crate::node::{Class, Context, Function, Node, Value};
use std::collections::HashMap;

#[derive(Clone)]
pub struct ClassDef {
    pub name: String,
    pub fields: Vec<String>,
    pub methods: HashMap<String, Function>,
}

impl Node for ClassDef {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        let class = Class {
            name: self.name.clone(),
            fields: self.fields.clone(),
            methods: self.methods.clone(),
        };
        ctx.classes.insert(self.name.clone(), class);
        Ok(Value::Void)
    }

    fn from_children(_rule: Rule, mut children: ParsedChildren) -> Box<dyn Node> {
        let name_node = children.take_child("name").unwrap();
        let name = name_node.text().unwrap_or_default();

        let mut fields = Vec::new();
        let mut methods = HashMap::new();

        for (_child_name, child) in children.remaining() {
            if let Some(field_def) = child
                .as_any()
                .downcast_ref::<crate::nodes::classes::FieldDef>()
            {
                fields.push(field_def.name.clone());
            } else if let Some(method_def) = child
                .as_any()
                .downcast_ref::<crate::nodes::classes::MethodDef>()
            {
                methods.insert(
                    method_def.name.clone(),
                    Function {
                        params: method_def.params.clone(),
                        body: method_def.body.clone(),
                    },
                );
            }
        }

        Box::new(ClassDef {
            name,
            fields,
            methods,
        })
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
