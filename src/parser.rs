use crate::grammar::{Grammar, Pattern};
use crate::node::Node;
use crate::nodes::{Int, Print, Program, Block, FunctionDef, FunctionCall};
use std::rc::Rc;
use regex::Regex;

pub struct Parser<'a> {
    grammar: &'a Grammar,
    input: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(grammar: &'a Grammar, input: &'a str) -> Self {
        Self { grammar, input }
    }

    pub fn parse(&self, rule_name: &str) -> Result<Box<dyn Node>, String> {
        let (node, _) = self.parse_rule(rule_name, 0)?;
        Ok(node)
    }

    fn parse_rule(&self, rule_name: &str, pos: usize) -> Result<(Box<dyn Node>, usize), String> {
        let rules = self.grammar.rules.get(rule_name).ok_or_else(|| format!("Rule not found: {}", rule_name))?;

        for rule in rules {
            match self.parse_sequence(&rule.patterns, pos) {
                Ok((children_with_names, new_pos)) => {
                    // Helper to extract children
                    // children_with_names is Vec<(Option<String>, Box<dyn Node>)>
                    
                    // Better strategy: Convert children to a workable structure
                    // We have ownership of children_with_names here.
                    let mut available_children: Vec<Option<(Option<String>, Box<dyn Node>)>> = children_with_names.into_iter().map(Some).collect();

                    let mut take_child = |name: &str| -> Option<Box<dyn Node>> {
                        // 1. Try named match
                        for item in available_children.iter_mut() {
                            if let Some((Some(n), _)) = item {
                                if n == name {
                                    let (_, node) = item.take().unwrap();
                                    return Some(node);
                                }
                            }
                        }
                        // 2. Fallback to first unnamed
                        for item in available_children.iter_mut() {
                            if let Some((None, _)) = item {
                                let (_, node) = item.take().unwrap();
                                return Some(node);
                            }
                        }
                        None
                    };
                    
                    // For Program, we just want all statements.
                    // But parse_sequence returns named tuples now.
                    // We can just extract all non-None items.
                    
                    let node: Box<dyn Node> = match rule_name {
                        "Program" => {
                            let stmts = available_children.into_iter().filter_map(|x| x.map(|(_, node)| node)).collect();
                            Box::new(Program { statements: stmts })
                        },
                        "Stmt" => available_children.into_iter().find_map(|x| x).map(|(_, node)| node).unwrap(),
                        "Print" => {
                            let expr = take_child("expression").unwrap(); // Fallback to first unnamed
                            Box::new(Print { expression: expr })
                        },
                        "Return" => {
                            let expr = take_child("expression").unwrap();
                            Box::new(crate::nodes::Return { expression: expr })
                        },
                        "Add" => {
                            let left = take_child("left").unwrap();
                            let right = take_child("right").unwrap();
                            Box::new(crate::nodes::term::Term { op: crate::nodes::term::AddOp::Add, left, right })
                        },
                        "Sub" => {
                            let left = take_child("left").unwrap();
                            let right = take_child("right").unwrap();
                            Box::new(crate::nodes::term::Term { op: crate::nodes::term::AddOp::Sub, left, right })
                        },
                        "Mul" => {
                            let left = take_child("left").unwrap();
                            let right = take_child("right").unwrap();
                            Box::new(crate::nodes::factor::Factor { op: crate::nodes::factor::MulOp::Mul, left, right })
                        },
                        "Div" => {
                            let left = take_child("left").unwrap();
                            let right = take_child("right").unwrap();
                            Box::new(crate::nodes::factor::Factor { op: crate::nodes::factor::MulOp::Div, left, right })
                        },
                        "IfElse" => {
                            let condition = take_child("condition").unwrap();
                            let then_block = take_child("then").unwrap();
                            let else_block = take_child("else"); // Optional? No, IfElse implies else exists in grammar usually.
                            // But my take_child returns Option.
                            // In IfElse rule: "if" condition:Expr then:Block "else" else:Block
                            // So else should be present.
                            Box::new(crate::nodes::if_node::If { 
                                condition, 
                                then_block, 
                                else_block: else_block 
                            })
                        },
                        "IfThen" => {
                            let condition = take_child("condition").unwrap();
                            let then_block = take_child("then").unwrap();
                            Box::new(crate::nodes::if_node::If { 
                                condition, 
                                then_block, 
                                else_block: None 
                            })
                        },
                        "True" => Box::new(crate::nodes::boolean::Boolean { value: true }),
                        "False" => Box::new(crate::nodes::boolean::Boolean { value: false }),
                        "Expr" | "Term" | "Factor" | "If" => available_children.into_iter().find_map(|x| x).map(|(_, node)| node).unwrap(),
                        "Int" => {
                             // Int logic remains similar, but we need to handle the child structure
                             // The child comes from Regex match in parse_sequence
                             // parse_sequence returns ([(None, RawTokenNode)], pos)
                             let child = available_children.into_iter().find_map(|x| x).map(|(_, node)| node).unwrap();
                             let text = child.text().unwrap_or_default();
                             Box::new(Int { value: text.parse().unwrap_or(0) })
                        },
                        "FunctionDef" => {
                            let name_node = take_child("name").unwrap();
                            let name = name_node.text().unwrap_or_default();
                            
                            // Parse params
                            let mut params = Vec::new();
                            if let Some(param_list) = take_child("params") {
                                // param_list is a Node, but we need to extract identifiers from it.
                                // The grammar will likely be: params:ParamList
                                // ParamList = Identifier ("," Identifier)*
                                // But our parser returns a Node tree.
                                // We need a way to traverse the ParamList node or parse it differently.
                                // Current parser structure makes it hard to traverse children of a child.
                                // However, `take_child` returns a Node. If we make ParamList a specific Node type that holds strings, we can downcast?
                                // Or we can rely on the fact that `available_children` contains all matches for the rule.
                                // But `params` is a named child.
                                
                                // Alternative: The grammar for FunctionDef will be:
                                // FunctionDef = "fn" name:Identifier "(" params:ParamList ")" "{" body:Block "}"
                                // ParamList will produce a node.
                                // Let's define a ParamList node? Or just parse it here?
                                // If ParamList is a rule, `take_child("params")` returns the node for that rule.
                                // If we define `ParamList` node to implement `text()` as comma separated? No.
                                
                                // Let's look at how `Block` works. It takes all statements.
                                // `ParamList` could be similar.
                                // Let's add `ParamList` to the match here.
                                // But `FunctionDef` needs to access the content of `ParamList`.
                                
                                // Hack: `ParamList` node will store the list of params.
                                // We need to cast `Box<dyn Node>` to `ParamList`.
                                // Rust doesn't support easy downcasting for trait objects without Any.
                                // Maybe we can add `as_any` to Node?
                                
                                // Or, we can just rely on `text()`? No.
                                
                                // Let's try to parse params manually from the input? No, that defeats the purpose.
                                
                                // Let's add `params()` method to Node trait?
                                // fn params(&self) -> Option<Vec<String>> { None }
                                
                                if let Some(p_list) = param_list.params() {
                                    params = p_list;
                                }
                            }
                            
                            let body = take_child("body").unwrap();
                            Box::new(FunctionDef { name, params, body: Rc::from(body) })
                        },
                        "FunctionCall" => {
                            let name_node = take_child("name").unwrap();
                            let name = name_node.text().unwrap_or_default();
                            
                            let mut args = Vec::new();
                            if let Some(arg_list) = take_child("args") {
                                // Similar issue for args.
                                // args:ArgList
                                // ArgList = Expr ("," Expr)*
                                // We need `args()` method on Node?
                                if arg_list.is_args() {
                                    args = arg_list.into_args();
                                }
                            }
                            
                            Box::new(FunctionCall { name, args })
                        },
                        "ParamList" => {
                             let mut params = Vec::new();
                             for item in available_children {
                                 if let Some((_, node)) = item {
                                     if let Some(text) = node.text() {
                                        // TODO: The , should not be hard coded
                                         if text != "," {
                                             params.push(text);
                                         }
                                     } else if let Some(sub_params) = node.params() {
                                         params.extend(sub_params);
                                     }
                                 }
                             }
                             Box::new(crate::nodes::ListNode { params: Some(params.clone()), args: None })
                        },
                        "ArgList" => {
                             let mut args = Vec::new();
                             for item in available_children {
                                 if let Some((_, node)) = item {
                                     if let Some(t) = node.text() {
                                        // TODO: The , should not be hard coded
                                         if t != "," {
                                             // Identifier or literal acting as arg
                                             // But wait, Identifier returns Variable node now.
                                             // Variable node is an Expr.
                                             // If it's a Variable node, we should treat it as an arg node, not just text.
                                             // But `text()` returns name for Variable.
                                             // We need to check if it's a "," literal.
                                             // If it is not ",", it is an arg.
                                             args.push(node);
                                         }
                                     } else if node.is_args() {
                                         args.extend(node.into_args());
                                     } else {
                                         args.push(node);
                                     }
                                 }
                             }
                             Box::new(crate::nodes::ListNode { params: None, args: Some(args) })
                        },
                        "Block" => {
                            let stmts = available_children.into_iter().filter_map(|x| x.map(|(_, node)| node)).collect();
                            Box::new(Block { statements: stmts })
                        },
                        "Identifier" => {
                             let child = available_children.into_iter().find_map(|x| x).map(|(_, node)| node).unwrap();
                             let name = child.text().unwrap_or_default();
                             Box::new(crate::nodes::Variable { name })
                        },
                        _ => panic!("Unknown rule: {}", rule_name),
                    };
                    return Ok((node, new_pos));
                }
                Err(_) => continue,
            }
        }

        Err(format!("No rules matched for {}", rule_name))
    }

    fn parse_sequence(&self, patterns: &[Pattern], mut pos: usize) -> Result<(Vec<(Option<String>, Box<dyn Node>)>, usize), String> {
        let mut children: Vec<(Option<String>, Box<dyn Node>)> = Vec::new();

        for pattern in patterns {
            pos = self.skip_whitespace(pos);
            match pattern {
                Pattern::Literal(s) => {
                    let len = s.len();
                    if self.input[pos..].starts_with(s) {
                        pos += len;
                    } else {
                        return Err(format!("Expected literal '{}'", s));
                    }
                }
                Pattern::Regex(r) => {
                    let re = Regex::new(&format!("^{}", r)).map_err(|e| e.to_string())?;
                    if let Some(mat) = re.find(&self.input[pos..]) {
                        let text = mat.as_str();
                        children.push((None, Box::new(RawTokenNode { text: text.to_string() })));
                        pos += mat.end();
                    } else {
                        return Err(format!("Expected regex match '{}'", r));
                    }
                }
                Pattern::RuleReference(name) => {
                    let (node, new_pos) = self.parse_rule(name, pos)?;
                    children.push((None, node));
                    pos = new_pos;
                }
                Pattern::Named(name, sub_pattern) => {
                    // Handle named pattern: recurse but wrap result with name
                    // But parse_sequence loop handles patterns.
                    // We need to handle the sub_pattern logic here.
                    // Refactor: Extract matching logic?
                    // Or just inline for now.
                    // Only Literal, Regex, RuleReference are likely inside Named.
                    match &**sub_pattern {
                        Pattern::Literal(s) => {
                             // Literals usually don't produce nodes.
                             // If named, maybe we should? But for now ignore name on literal?
                             // Or create a TokenNode?
                             let len = s.len();
                             if self.input[pos..].starts_with(s) {
                                 pos += len;
                             } else {
                                 return Err(format!("Expected literal '{}'", s));
                             }
                        },
                        Pattern::Regex(r) => {
                            let re = Regex::new(&format!("^{}", r)).map_err(|e| e.to_string())?;
                            if let Some(mat) = re.find(&self.input[pos..]) {
                                let text = mat.as_str();
                                children.push((Some(name.clone()), Box::new(RawTokenNode { text: text.to_string() })));
                                pos += mat.end();
                            } else {
                                return Err(format!("Expected regex match '{}'", r));
                            }
                        },
                        Pattern::RuleReference(ref_name) => {
                             let (node, new_pos) = self.parse_rule(ref_name, pos)?;
                             children.push((Some(name.clone()), node));
                             pos = new_pos;
                        },
                        _ => return Err("Unsupported pattern inside Named".to_string()),
                    }
                }
                Pattern::Star(sub_pattern) => {
                    loop {
                        match &**sub_pattern {
                            Pattern::RuleReference(name) => {
                                match self.parse_rule(name, pos) {
                                    Ok((node, new_pos)) => {
                                        children.push((None, node));
                                        pos = new_pos;
                                    }
                                    Err(_) => break,
                                }
                            }
                            _ => return Err("Only *Rule supported for now".to_string()),
                        }
                    }
                }
            }
        }

        Ok((children, pos))
    }

    fn skip_whitespace(&self, mut pos: usize) -> usize {
        while pos < self.input.len() && self.input[pos..].chars().next().unwrap().is_whitespace() {
            pos += 1;
        }
        pos
    }
}

struct RawTokenNode {
    text: String,
}

impl Node for RawTokenNode {
    fn run(&self, _ctx: &mut crate::node::Context) -> crate::node::Value {
        crate::node::Value::Void
    }
    
    fn text(&self) -> Option<String> {
        Some(self.text.clone())
    }
}
