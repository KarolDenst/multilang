use crate::error::{ParseError, RuntimeError};
use crate::grammar::{Grammar, Pattern};
use crate::node::{Node, ParsedChildren, Value};
use crate::nodes::list_node::ElementsNode;
use crate::nodes::{
    ArgListNode, Assignment, Block, Comparison, Factor, ForNode, FunctionCall, FunctionDef, If,
    ListNode, Literal, Logical, Program, Return, Term, Unary, Variable, WhileNode,
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

    pub fn parse(&self, rule_name: &str) -> Result<Box<dyn Node>, ParseError> {
        let (node, pos) = self.parse_rule(rule_name, 0)?;
        let final_pos = self.skip_whitespace(pos);
        if final_pos < self.input.len() {
            let (line, col, line_content) = self.get_location(final_pos);
            return Err(ParseError {
                message: format!("Unexpected token at end of input"),
                line,
                column: col,
                line_content,
            });
        }
        Ok(node)
    }

    fn get_location(&self, pos: usize) -> (usize, usize, String) {
        let mut line = 1;
        let mut col = 1;
        let mut last_newline = 0;
        for (i, c) in self.input.char_indices() {
            if i == pos {
                break;
            }
            if c == '\n' {
                line += 1;
                col = 1;
                last_newline = i + 1;
            } else {
                col += 1;
            }
        }
        // Extract line content
        let end_of_line = self.input[last_newline..]
            .find('\n')
            .map(|i| last_newline + i)
            .unwrap_or(self.input.len());
        let line_content = self.input[last_newline..end_of_line].to_string();
        (line, col, line_content)
    }

    fn parse_rule(
        &self,
        rule_name: &str,
        pos: usize,
    ) -> Result<(Box<dyn Node>, usize), ParseError> {
        // Check cache
        let key = (rule_name.to_string(), pos);
        if let Some(cached) = self.cache.borrow().get(&key) {
            return match cached {
                Some((node, new_pos)) => Ok((node.box_clone(), *new_pos)),
                None => {
                    let (line, col, line_content) = self.get_location(pos);
                    Err(ParseError {
                        message: format!("Parsing failed for rule {} at pos {}", rule_name, pos),
                        line,
                        column: col,
                        line_content,
                    })
                }
            };
        }

        let rules = self.grammar.rules.get(rule_name).ok_or_else(|| {
            let (line, col, line_content) = self.get_location(pos);
            ParseError {
                message: format!("Rule not found: {}", rule_name),
                line,
                column: col,
                line_content,
            }
        })?;

        for rule in rules {
            match self.parse_sequence(&rule.patterns, pos) {
                Ok((children_with_names, new_pos)) => {
                    // Helper to extract children
                    // children_with_names is Vec<(Option<String>, Box<dyn Node>)>

                    // Better strategy: Convert children to a workable structure
                    // We have ownership of children_with_names here.
                    let (line, _, _) = self.get_location(pos);
                    let parsed_children = ParsedChildren::new(children_with_names, line);

                    let node: Box<dyn Node> = match rule_name {
                        "Program" => Program::from_children(rule_name, parsed_children),
                        "Stmt" => parsed_children.remaining().into_iter().next().unwrap().1,
                        "Assignment" => Assignment::from_children(rule_name, parsed_children),
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
                            ArgListNode::from_children(rule_name, parsed_children)
                        }
                        "ListLiteral" => ListNode::from_children(rule_name, parsed_children),
                        "Elements" => ElementsNode::from_children(rule_name, parsed_children),
                        "ForLoop" => ForNode::from_children(rule_name, parsed_children),
                        "WhileLoop" => WhileNode::from_children(rule_name, parsed_children),
                        "Block" => Block::from_children(rule_name, parsed_children),
                        "Identifier" => Variable::from_children(rule_name, parsed_children),
                        "Expr" | "Atom" | "If" | "UnaryOp" | "Eq" | "Neq" | "Lt" | "Gt" | "Add"
                        | "Sub" | "Mul" | "Div" | "Mod" => {
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
        let (line, col, line_content) = self.get_location(pos);
        Err(ParseError {
            message: format!("No rules matched for {}", rule_name),
            line,
            column: col,
            line_content,
        })
    }

    fn parse_sequence(
        &self,
        patterns: &[Pattern],
        mut pos: usize,
    ) -> Result<(Vec<(Option<String>, Box<dyn Node>)>, usize), ParseError> {
        let mut children: Vec<(Option<String>, Box<dyn Node>)> = Vec::new();

        for pattern in patterns {
            pos = self.skip_whitespace(pos);
            match pattern {
                Pattern::Literal(s) => {
                    let len = s.len();
                    if self.input[pos..].starts_with(s) {
                        pos += len;
                    } else {
                        let (line, col, line_content) = self.get_location(pos);
                        return Err(ParseError {
                            message: format!("Expected literal '{}'", s),
                            line,
                            column: col,
                            line_content,
                        });
                    }
                }
                Pattern::Regex(r) => {
                    let re = Regex::new(&format!("^{}", r)).map_err(|e| {
                        let (line, col, line_content) = self.get_location(pos);
                        ParseError {
                            message: e.to_string(),
                            line,
                            column: col,
                            line_content,
                        }
                    })?;
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
                        let (line, col, line_content) = self.get_location(pos);
                        return Err(ParseError {
                            message: format!("Expected regex match '{}'", r),
                            line,
                            column: col,
                            line_content,
                        });
                    }
                }
                Pattern::RuleReference(name) => {
                    let (node, new_pos) = self.parse_rule(name, pos)?;
                    children.push((None, node));
                    pos = new_pos;
                }
                Pattern::Named(name, sub_pattern) => match &**sub_pattern {
                    Pattern::Literal(s) => {
                        let len = s.len();
                        if self.input[pos..].starts_with(s) {
                            pos += len;
                        } else {
                            let (line, col, line_content) = self.get_location(pos);
                            return Err(ParseError {
                                message: format!("Expected literal '{}'", s),
                                line,
                                column: col,
                                line_content,
                            });
                        }
                    }
                    Pattern::Regex(r) => {
                        let re = Regex::new(&format!("^{}", r)).map_err(|e| {
                            let (line, col, line_content) = self.get_location(pos);
                            ParseError {
                                message: e.to_string(),
                                line,
                                column: col,
                                line_content,
                            }
                        })?;
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
                            let (line, col, line_content) = self.get_location(pos);
                            return Err(ParseError {
                                message: format!("Expected regex match '{}'", r),
                                line,
                                column: col,
                                line_content,
                            });
                        }
                    }
                    Pattern::RuleReference(ref_name) => {
                        let (node, new_pos) = self.parse_rule(ref_name, pos)?;
                        children.push((Some(name.clone()), node));
                        pos = new_pos;
                    }
                    _ => {
                        let (line, col, line_content) = self.get_location(pos);
                        return Err(ParseError {
                            message: "Unsupported pattern inside Named".to_string(),
                            line,
                            column: col,
                            line_content,
                        });
                    }
                },
                Pattern::Star(sub_pattern) => loop {
                    match &**sub_pattern {
                        Pattern::RuleReference(name) => match self.parse_rule(name, pos) {
                            Ok((node, new_pos)) => {
                                children.push((None, node));
                                pos = new_pos;
                            }
                            Err(_) => break,
                        },
                        _ => {
                            let (line, col, line_content) = self.get_location(pos);
                            return Err(ParseError {
                                message: "Only *Rule supported for now".to_string(),
                                line,
                                column: col,
                                line_content,
                            });
                        }
                    }
                },
            }
        }

        Ok((children, pos))
    }

    fn skip_whitespace(&self, mut pos: usize) -> usize {
        loop {
            let mut changed = false;
            // Skip whitespace
            while pos < self.input.len()
                && self.input[pos..].chars().next().unwrap().is_whitespace()
            {
                pos += 1;
                changed = true;
            }
            // Skip comments
            if pos < self.input.len() && self.input[pos..].starts_with("//") {
                while pos < self.input.len() && self.input[pos..].chars().next().unwrap() != '\n' {
                    pos += 1;
                }
                changed = true;
            }

            if !changed {
                break;
            }
        }
        pos
    }
}

#[derive(Clone)]
struct RawTokenNode {
    text: String,
}

impl Node for RawTokenNode {
    fn run(&self, _ctx: &mut crate::node::Context) -> Result<Value, RuntimeError> {
        Ok(Value::Void)
    }

    fn text(&self) -> Option<String> {
        Some(self.text.clone())
    }

    fn from_children(_rule_name: &str, _children: ParsedChildren) -> Box<dyn Node> {
        panic!("RawTokenNode should not be created from children");
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(RawTokenNode {
            text: self.text.clone(),
        })
    }
}
