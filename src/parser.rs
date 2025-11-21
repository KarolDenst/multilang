use crate::grammar::{Grammar, Pattern};
use crate::node::Node;
use crate::nodes::{
    Block, Comparison, Factor, FunctionCall, FunctionDef, If, ListNode, Literal, Logical, Program,
    Return, Term, Unary, Variable,
};
use regex::Regex;

use std::cell::RefCell;
use std::collections::HashMap;

pub struct Parser<'a> {
    grammar: &'a Grammar,
    input: &'a str,
    cache: RefCell<HashMap<(String, usize), Option<(Box<dyn Node>, usize)>>>,
}

impl<'a> Parser<'a> {
    pub fn new(grammar: &'a Grammar, input: &'a str) -> Self {
        Self {
            grammar,
            input,
            cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn parse(&self, rule_name: &str) -> Result<Box<dyn Node>, String> {
        let (node, _) = self.parse_rule(rule_name, 0)?;
        Ok(node)
    }

    fn parse_rule(&self, rule_name: &str, pos: usize) -> Result<(Box<dyn Node>, usize), String> {
        // Check cache
        let key = (rule_name.to_string(), pos);
        if let Some(cached) = self.cache.borrow().get(&key) {
            return match cached {
                Some((node, new_pos)) => Ok((node.box_clone(), *new_pos)),
                None => Err(format!(
                    "Parsing failed for rule {} at pos {}",
                    rule_name, pos
                )),
            };
        }

        let rules = self
            .grammar
            .rules
            .get(rule_name)
            .ok_or_else(|| format!("Rule not found: {}", rule_name))?;

        for rule in rules {
            match self.parse_sequence(&rule.patterns, pos) {
                Ok((children_with_names, new_pos)) => {
                    // if rule_name == "Comparison" {
                    //    println!("Matched Comparison with {} children", children_with_names.len());
                    // }
                    // Helper to extract children
                    // children_with_names is Vec<(Option<String>, Box<dyn Node>)>

                    // Better strategy: Convert children to a workable structure
                    // We have ownership of children_with_names here.
                    let parsed_children = crate::node::ParsedChildren::new(children_with_names);

                    let node: Box<dyn Node> = match rule_name {
                        "Program" => Program::from_children(rule_name, parsed_children),
                        "Stmt" => parsed_children.remaining().into_iter().next().unwrap().1,
                        "Return" => Return::from_children(rule_name, parsed_children),
                        "Comparison" => Comparison::from_children(rule_name, parsed_children),
                        "LogicalOr" | "LogicalAnd" => {
                            Logical::from_children(rule_name, parsed_children)
                        }
                        "Term" => Term::from_children(rule_name, parsed_children),
                        "Factor" => Factor::from_children(rule_name, parsed_children),
                        "Unary" => Unary::from_children(rule_name, parsed_children),
                        "IfElse" | "IfThen" => If::from_children(rule_name, parsed_children),
                        "Int" | "Float" | "String" | "True" | "False" => {
                            Literal::from_children(rule_name, parsed_children)
                        }
                        "FunctionDef" => FunctionDef::from_children(rule_name, parsed_children),
                        "FunctionCall" => FunctionCall::from_children(rule_name, parsed_children),
                        "ParamList" | "ArgList" => {
                            ListNode::from_children(rule_name, parsed_children)
                        }
                        "Block" => Block::from_children(rule_name, parsed_children),
                        "Identifier" => Variable::from_children(rule_name, parsed_children),
                        "Expr" | "Atom" | "If" | "UnaryOp" | "Eq" | "Neq" | "Lt" | "Gt" | "Add"
                        | "Sub" | "Mul" | "Div" => {
                            parsed_children.remaining().into_iter().next().unwrap().1
                        }
                        _ => panic!("Unknown rule: {}", rule_name),
                    };

                    // Cache success
                    self.cache
                        .borrow_mut()
                        .insert(key, Some((node.box_clone(), new_pos)));
                    return Ok((node, new_pos));
                }
                Err(_) => continue,
            }
        }

        // Cache failure
        self.cache.borrow_mut().insert(key, None);
        Err(format!("No rules matched for {}", rule_name))
    }

    fn parse_sequence(
        &self,
        patterns: &[Pattern],
        mut pos: usize,
    ) -> Result<(Vec<(Option<String>, Box<dyn Node>)>, usize), String> {
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
                        children.push((
                            None,
                            Box::new(RawTokenNode {
                                text: text.to_string(),
                            }),
                        ));
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
                        }
                        Pattern::Regex(r) => {
                            let re = Regex::new(&format!("^{}", r)).map_err(|e| e.to_string())?;
                            if let Some(mat) = re.find(&self.input[pos..]) {
                                let text = mat.as_str();
                                children.push((
                                    Some(name.clone()),
                                    Box::new(RawTokenNode {
                                        text: text.to_string(),
                                    }),
                                ));
                                pos += mat.end();
                            } else {
                                return Err(format!("Expected regex match '{}'", r));
                            }
                        }
                        Pattern::RuleReference(ref_name) => {
                            let (node, new_pos) = self.parse_rule(ref_name, pos)?;
                            children.push((Some(name.clone()), node));
                            pos = new_pos;
                        }
                        _ => return Err("Unsupported pattern inside Named".to_string()),
                    }
                }
                Pattern::Star(sub_pattern) => loop {
                    match &**sub_pattern {
                        Pattern::RuleReference(name) => match self.parse_rule(name, pos) {
                            Ok((node, new_pos)) => {
                                children.push((None, node));
                                pos = new_pos;
                            }
                            Err(_) => break,
                        },
                        _ => return Err("Only *Rule supported for now".to_string()),
                    }
                },
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

#[derive(Clone)]
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

    fn from_children(_rule_name: &str, _children: crate::node::ParsedChildren) -> Box<dyn Node> {
        panic!("RawTokenNode should not be created from children");
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(RawTokenNode {
            text: self.text.clone(),
        })
    }
}
