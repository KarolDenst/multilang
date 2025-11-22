use crate::error::RuntimeError;
use crate::grammar::Rule;
use crate::node::ParsedChildren;
use crate::node::{Context, Node, Value};

pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Box<dyn Node>>,
    pub line: usize,
}

impl Node for FunctionCall {
    fn run(&self, ctx: &mut Context) -> Result<Value, RuntimeError> {
        // 1. Check built-ins
        let builtin = ctx.builtins.get(&self.name).copied();
        if let Some(func) = builtin {
            let mut arg_values = Vec::new();
            for arg in &self.args {
                arg_values.push(arg.run(ctx)?);
            }
            return func(arg_values);
        }

        // 2. Check user-defined functions
        if let Some(func) = ctx.functions.get(&self.name) {
            let func_params = func.params.clone();
            let func_body = func.body.clone();

            // Evaluate arguments in current context
            let mut arg_values = Vec::new();
            for arg in &self.args {
                arg_values.push(arg.run(ctx)?);
            }

            if arg_values.len() != func_params.len() {
                return Err(RuntimeError {
                    message: format!(
                        "Function '{}' expects {} arguments, got {}",
                        self.name,
                        func_params.len(),
                        arg_values.len()
                    ),
                    stack_trace: vec![format!("at {}:{}", self.name, self.line)],
                });
            }

            // Create new context for function execution
            // We need to copy functions to the new context so it can call other functions
            // Variables are NOT copied (scoping)
            let mut new_ctx = Context::new();
            new_ctx.functions = ctx.functions.clone(); // Shallow clone of HashMap, Rc are cheap
            new_ctx.builtins = ctx.builtins.clone(); // Also copy builtins

            // Bind arguments to parameters
            for (param, value) in func_params.iter().zip(arg_values.into_iter()) {
                new_ctx.variables.insert(param.clone(), value);
            }

            match func_body.run(&mut new_ctx) {
                Ok(val) => Ok(val),
                Err(mut err) => {
                    err.stack_trace
                        .push(format!("at {}:{}", self.name, self.line));
                    Err(err)
                }
            }
        } else {
            Err(RuntimeError {
                message: format!("Function '{}' not found", self.name),
                stack_trace: vec![format!("at {}:{}", self.name, self.line)],
            })
        }
    }

    fn from_children(_rule: Rule, mut children: ParsedChildren) -> Box<dyn Node> {
        let line = children.line;
        let name_node = children.take_child("name").unwrap();
        let name = name_node.text().unwrap_or_default();

        let mut args = Vec::new();
        if let Some(arg_list) = children.take_child("args")
            && arg_list.is_args() {
                args = arg_list.into_args();
            }

        Box::new(FunctionCall { name, args, line })
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(FunctionCall {
            name: self.name.clone(),
            args: self.args.iter().map(|a| a.box_clone()).collect(),
            line: self.line,
        })
    }
}
