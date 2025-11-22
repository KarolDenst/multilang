use crate::error::RuntimeError;
use crate::grammar::Rule;
use crate::node::ParsedChildren;
use crate::node::{Context, Node, Value};

#[derive(Clone)]
pub struct MethodCall {
    pub object: Box<dyn Node>,
    pub method_name: String,
    pub args: Vec<Box<dyn Node>>,
}

impl Node for MethodCall {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        // 1. Evaluate object
        let obj_val = self.object.run(ctx)?;

        if let Value::Object(obj_rc) = obj_val {
            let obj = obj_rc.borrow();

            // 2. Look up class
            let class = ctx
                .classes
                .get(&obj.class_name)
                .ok_or_else(|| RuntimeError {
                    message: format!("Class '{}' not found", obj.class_name),
                    stack_trace: vec![],
                })?;

            // 3. Look up method
            let method = class
                .methods
                .get(&self.method_name)
                .ok_or_else(|| RuntimeError {
                    message: format!(
                        "Method '{}' not found in class '{}'",
                        self.method_name, obj.class_name
                    ),
                    stack_trace: vec![],
                })?
                .clone();

            // 4. Evaluate args
            let mut arg_values = Vec::new();
            for arg in &self.args {
                arg_values.push(arg.run(ctx)?);
            }

            // 5. Check arg count
            if arg_values.len() != method.params.len() {
                return Err(RuntimeError {
                    message: format!(
                        "Method '{}' expects {} arguments, got {}",
                        self.method_name,
                        method.params.len(),
                        arg_values.len()
                    ),
                    stack_trace: vec![],
                });
            }

            // 6. Create new context
            // We need to capture functions/classes/builtins from current context?
            // Yes, methods should be able to call other functions/classes.
            // But variables should be local + 'this'.
            // Actually, `FunctionCall` creates a new context with global functions/builtins.
            // Here we should do similar.

            let mut new_ctx = Context::new();
            new_ctx.functions = ctx.functions.clone(); // Should be reference or COW? For now clone is fine (expensive but correct)
            new_ctx.classes = ctx.classes.clone();
            new_ctx.builtins = ctx.builtins.clone();

            // Bind params
            for (i, param) in method.params.iter().enumerate() {
                new_ctx
                    .variables
                    .insert(param.clone(), arg_values[i].clone());
            }

            // Bind 'this'
            // We need to drop the borrow on obj_rc before inserting into new_ctx?
            // obj_rc is Rc<RefCell<Object>>. Value::Object holds it.
            // We borrowed it as `obj`.
            // We need to clone obj_rc.
            drop(obj); // Drop borrow
            new_ctx
                .variables
                .insert("this".to_string(), Value::Object(obj_rc.clone()));

            // 7. Run body
            method.body.run(&mut new_ctx)
        } else {
            return Err(RuntimeError {
                message: format!("Cannot call method '{}' on non-object", self.method_name),
                stack_trace: vec![],
            });
        }
    }

    fn from_children(_rule: Rule, mut children: ParsedChildren) -> Box<dyn Node> {
        let object = children.take_child("object").unwrap();
        let method_node = children.take_child("method").unwrap();
        let method_name = method_node.text().unwrap_or_default();

        let mut args = Vec::new();
        if let Some(arg_list) = children.take_child("args") {
            args = arg_list.into_args();
        }

        Box::new(MethodCall {
            object,
            method_name,
            args,
        })
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
