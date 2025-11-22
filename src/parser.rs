use crate::error::{ParseError, RuntimeError};
use crate::grammar::{Grammar, Pattern, Rule};
use crate::node::{Node, ParsedChildren, Value};
use crate::nodes::{
    ArgListNode, Assignment, Block, ClassDef, Comparison, ElementsNode, Factor, FieldDef, ForNode,
    FunctionCall, FunctionDef, If, ListNode, Literal, Logical, MapEntriesNode, MapEntryNode,
    MapNode, MemberAccess, MethodCall, MethodDef, NewExpr, PostfixNode, PostfixSuffixNode, Program,
    Return, SelfReference, Term, Unary, Variable, WhileNode,
};
use regex::Regex;

use std::cell::RefCell;
use std::collections::HashMap;

type CacheKey = (Rule, usize);
type CacheEntry = Option<(Box<dyn Node>, usize)>;
type ParsedChild = (Option<String>, Box<dyn Node>);
type ParseSequenceResult = Result<(Vec<ParsedChild>, usize), ParseError>;

pub struct Parser<'a> {
    grammar: &'a Grammar,
    input: &'a str,
    cache: RefCell<HashMap<CacheKey, CacheEntry>>,
}

impl<'a> Parser<'a> {
    pub fn new(grammar: &'a Grammar, input: &'a str) -> Self {
        Self {
            grammar,
            input,
            cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn parse(&self, rule_name: Rule) -> Result<Box<dyn Node>, ParseError> {
        let (node, pos) = self.parse_rule(rule_name, 0)?;
        let final_pos = self.skip_whitespace(pos);
        if final_pos < self.input.len() {
            let (line, col, line_content) = self.get_location(final_pos);
            return Err(ParseError {
                message: "Unexpected token at end of input".to_string(),
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
        rule_name: Rule,
        pos: usize,
    ) -> Result<(Box<dyn Node>, usize), ParseError> {
        // Check cache
        let key = (rule_name, pos);
        if let Some(cached) = self.cache.borrow().get(&key) {
            return match cached {
                Some((node, new_pos)) => Ok((node.box_clone(), *new_pos)),
                None => {
                    let (line, col, line_content) = self.get_location(pos);
                    Err(ParseError {
                        message: format!("Parsing failed for rule {:?} at pos {}", rule_name, pos),
                        line,
                        column: col,
                        line_content,
                    })
                }
            };
        }

        let rules = self.grammar.rules.get(&rule_name).ok_or_else(|| {
            let (line, col, line_content) = self.get_location(pos);
            ParseError {
                message: format!("Rule not found: {:?}", rule_name),
                line,
                column: col,
                line_content,
            }
        })?;

        for rule in rules {
            match self.parse_sequence(&rule.patterns, pos) {
                Ok((children_with_names, new_pos)) => {
                    let (line, _, _) = self.get_location(pos);
                    let parsed_children = ParsedChildren::new(children_with_names, line);

                    let node: Box<dyn Node> = match rule_name {
                        Rule::Program => Program::from_children(rule_name, parsed_children),
                        Rule::Stmt => parsed_children.remaining().into_iter().next().unwrap().1,
                        Rule::Assignment => Assignment::from_children(rule_name, parsed_children),
                        Rule::Return => Return::from_children(rule_name, parsed_children),
                        Rule::Comparison => Comparison::from_children(rule_name, parsed_children),
                        Rule::LogicalOr | Rule::LogicalAnd => {
                            Logical::from_children(rule_name, parsed_children)
                        }
                        Rule::Term => Term::from_children(rule_name, parsed_children),
                        Rule::Factor => Factor::from_children(rule_name, parsed_children),
                        Rule::Unary => Unary::from_children(rule_name, parsed_children),
                        Rule::IfElse | Rule::IfThen => {
                            If::from_children(rule_name, parsed_children)
                        }
                        Rule::Int | Rule::Float | Rule::String | Rule::True | Rule::False => {
                            Literal::from_children(rule_name, parsed_children)
                        }
                        Rule::FunctionDef => FunctionDef::from_children(rule_name, parsed_children),
                        Rule::FunctionCall => {
                            FunctionCall::from_children(rule_name, parsed_children)
                        }
                        Rule::ParamList | Rule::ArgList => {
                            ArgListNode::from_children(rule_name, parsed_children)
                        }
                        Rule::ListLiteral => ListNode::from_children(rule_name, parsed_children),
                        Rule::Elements => ElementsNode::from_children(rule_name, parsed_children),
                        Rule::MapLiteral => MapNode::from_children(rule_name, parsed_children),
                        Rule::MapEntries => {
                            MapEntriesNode::from_children(rule_name, parsed_children)
                        }
                        Rule::MapEntry => MapEntryNode::from_children(rule_name, parsed_children),
                        Rule::ForLoop => ForNode::from_children(rule_name, parsed_children),
                        Rule::WhileLoop => WhileNode::from_children(rule_name, parsed_children),
                        Rule::Block => Block::from_children(rule_name, parsed_children),
                        Rule::Identifier => Variable::from_children(rule_name, parsed_children),
                        Rule::ClassDef => ClassDef::from_children(rule_name, parsed_children),
                        Rule::ClassMember => {
                            // ClassMember is a wrapper, return the child directly
                            parsed_children.remaining().into_iter().next().unwrap().1
                        }
                        Rule::FieldDef => FieldDef::from_children(rule_name, parsed_children),
                        Rule::MethodDef => MethodDef::from_children(rule_name, parsed_children),
                        Rule::NewExpr => NewExpr::from_children(rule_name, parsed_children),
                        Rule::MemberAccess => {
                            MemberAccess::from_children(rule_name, parsed_children)
                        }
                        Rule::MethodCall => MethodCall::from_children(rule_name, parsed_children),
                        Rule::SelfReference => {
                            SelfReference::from_children(rule_name, parsed_children)
                        }
                        Rule::Postfix => PostfixNode::from_children(rule_name, parsed_children),
                        Rule::PostfixSuffix => {
                            PostfixSuffixNode::from_children(rule_name, parsed_children)
                        }
                        Rule::Expr
                        | Rule::Atom
                        | Rule::If
                        | Rule::UnaryOp
                        | Rule::Eq
                        | Rule::Neq
                        | Rule::Lt
                        | Rule::Gt
                        | Rule::Add
                        | Rule::Sub
                        | Rule::Mul
                        | Rule::Div
                        | Rule::Mod
                        | Rule::Key => parsed_children.remaining().into_iter().next().unwrap().1,
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
            message: format!("No rules matched for {:?}", rule_name),
            line,
            column: col,
            line_content,
        })
    }

    fn parse_sequence(&self, patterns: &[Pattern], mut pos: usize) -> ParseSequenceResult {
        let mut children: Vec<ParsedChild> = Vec::new();

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
                Pattern::RuleReference(rule) => {
                    let (node, new_pos) = self.parse_rule(*rule, pos)?;
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
                    Pattern::RuleReference(ref_rule) => {
                        let (node, new_pos) = self.parse_rule(*ref_rule, pos)?;
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
                        Pattern::RuleReference(rule) => match self.parse_rule(*rule, pos) {
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
                while pos < self.input.len() && !self.input[pos..].starts_with('\n') {
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

    fn from_children(_rule: Rule, _children: ParsedChildren) -> Box<dyn Node> {
        panic!("RawTokenNode should not be created from children");
    }

    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(RawTokenNode {
            text: self.text.clone(),
        })
    }
}
