use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rule {
    Program,
    Stmt,
    Assignment,
    Return,
    Comparison,
    LogicalOr,
    LogicalAnd,
    Term,
    Factor,
    Unary,
    IfElse,
    IfThen,
    Int,
    Float,
    String,
    True,
    False,
    FunctionDef,
    FunctionCall,
    ParamList,
    ArgList,
    ListLiteral,
    Elements,
    MapLiteral,
    MapEntries,
    MapEntry,
    ForLoop,
    WhileLoop,
    Block,
    Identifier,
    Expr,
    Atom,
    If,
    UnaryOp,
    Eq,
    Neq,
    Lt,
    Gt,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Key,
    ClassDef,
    ClassMember,
    FieldDef,
    MethodDef,
    NewExpr,
    MemberAccess,
    MethodCall,
    SelfReference,
    Postfix,
    PostfixSuffix,
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Program" => Ok(Rule::Program),
            "Stmt" => Ok(Rule::Stmt),
            "Assignment" => Ok(Rule::Assignment),
            "Return" => Ok(Rule::Return),
            "Comparison" => Ok(Rule::Comparison),
            "LogicalOr" => Ok(Rule::LogicalOr),
            "LogicalAnd" => Ok(Rule::LogicalAnd),
            "Term" => Ok(Rule::Term),
            "Factor" => Ok(Rule::Factor),
            "Unary" => Ok(Rule::Unary),
            "IfElse" => Ok(Rule::IfElse),
            "IfThen" => Ok(Rule::IfThen),
            "Int" => Ok(Rule::Int),
            "Float" => Ok(Rule::Float),
            "String" => Ok(Rule::String),
            "True" => Ok(Rule::True),
            "False" => Ok(Rule::False),
            "FunctionDef" => Ok(Rule::FunctionDef),
            "FunctionCall" => Ok(Rule::FunctionCall),
            "ParamList" => Ok(Rule::ParamList),
            "ArgList" => Ok(Rule::ArgList),
            "ListLiteral" => Ok(Rule::ListLiteral),
            "Elements" => Ok(Rule::Elements),
            "MapLiteral" => Ok(Rule::MapLiteral),
            "MapEntries" => Ok(Rule::MapEntries),
            "MapEntry" => Ok(Rule::MapEntry),
            "ForLoop" => Ok(Rule::ForLoop),
            "WhileLoop" => Ok(Rule::WhileLoop),
            "Block" => Ok(Rule::Block),
            "Identifier" => Ok(Rule::Identifier),
            "Expr" => Ok(Rule::Expr),
            "Atom" => Ok(Rule::Atom),
            "If" => Ok(Rule::If),
            "UnaryOp" => Ok(Rule::UnaryOp),
            "Eq" => Ok(Rule::Eq),
            "Neq" => Ok(Rule::Neq),
            "Lt" => Ok(Rule::Lt),
            "Gt" => Ok(Rule::Gt),
            "Add" => Ok(Rule::Add),
            "Sub" => Ok(Rule::Sub),
            "Mul" => Ok(Rule::Mul),
            "Div" => Ok(Rule::Div),
            "Mod" => Ok(Rule::Mod),
            "Key" => Ok(Rule::Key),
            "ClassDef" => Ok(Rule::ClassDef),
            "ClassMember" => Ok(Rule::ClassMember),
            "FieldDef" => Ok(Rule::FieldDef),
            "MethodDef" => Ok(Rule::MethodDef),
            "NewExpr" => Ok(Rule::NewExpr),
            "MemberAccess" => Ok(Rule::MemberAccess),
            "MethodCall" => Ok(Rule::MethodCall),
            "SelfReference" => Ok(Rule::SelfReference),
            "Postfix" => Ok(Rule::Postfix),
            "PostfixSuffix" => Ok(Rule::PostfixSuffix),
            _ => Err(format!("Unknown rule: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(String),
    RuleReference(Rule),
    Regex(String),
    Star(Box<Pattern>),
    Named(String, Box<Pattern>), // name:Pattern
}

#[derive(Debug, Clone)]
pub struct Production {
    pub rule: Rule,
    pub patterns: Vec<Pattern>,
}

#[derive(Debug, Clone)]
pub struct Grammar {
    pub rules: HashMap<Rule, Vec<Production>>,
}

impl Default for Grammar {
    fn default() -> Self {
        Self::new()
    }
}

impl Grammar {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, rule: Rule, patterns: Vec<Pattern>) {
        self.rules
            .entry(rule)
            .or_default()
            .push(Production { rule, patterns });
    }

    pub fn parse(input: &str) -> Self {
        let mut grammar = Grammar::new();

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            let (name_str, rhs) = match line.split_once('=') {
                Some((n, r)) => (n.trim(), r.trim().trim_end_matches(';')),
                None => continue,
            };

            let rule = match Rule::from_str(name_str) {
                Ok(r) => r,
                Err(e) => panic!("Error parsing grammar: {}", e),
            };

            let mut alternatives = Vec::new();
            let mut current_alt = String::new();
            let mut in_quote = false;
            let mut in_regex = false;

            for c in rhs.chars() {
                if c == '"' && !in_regex {
                    in_quote = !in_quote;
                } else if c == '[' && !in_quote {
                    in_regex = true;
                } else if c == ']' && !in_quote {
                    in_regex = false;
                }

                if c == '|' && !in_quote && !in_regex {
                    alternatives.push(current_alt.trim().to_string());
                    current_alt.clear();
                } else {
                    current_alt.push(c);
                }
            }
            alternatives.push(current_alt.trim().to_string());

            for alternative in alternatives {
                let mut patterns = Vec::new();
                for token in alternative.split_whitespace() {
                    if token.starts_with('"') && token.ends_with('"') {
                        patterns.push(Pattern::Literal(token[1..token.len() - 1].to_string()));
                    } else if token.starts_with('[') && token.ends_with(']') {
                        patterns.push(Pattern::Regex(token[1..token.len() - 1].to_string()));
                    } else if let Some(sub_token) = token.strip_suffix('*') {
                        let sub_rule = Rule::from_str(sub_token)
                            .unwrap_or_else(|_| panic!("Unknown rule in pattern: {}", sub_token));
                        patterns.push(Pattern::Star(Box::new(Pattern::RuleReference(sub_rule))));
                    } else if let Some(idx) = token.find(':') {
                        // Handle name:Pattern
                        let field_name = &token[..idx];
                        let sub_token = &token[idx + 1..];

                        let sub_pattern = if sub_token.starts_with('"') && sub_token.ends_with('"')
                        {
                            Pattern::Literal(sub_token[1..sub_token.len() - 1].to_string())
                        } else if sub_token.starts_with('[') && sub_token.ends_with(']') {
                            Pattern::Regex(sub_token[1..sub_token.len() - 1].to_string())
                        } else {
                            let sub_rule = Rule::from_str(sub_token).unwrap_or_else(|_| {
                                panic!("Unknown rule in pattern: {}", sub_token)
                            });
                            Pattern::RuleReference(sub_rule)
                        };

                        patterns.push(Pattern::Named(
                            field_name.to_string(),
                            Box::new(sub_pattern),
                        ));
                    } else {
                        let sub_rule = Rule::from_str(token)
                            .unwrap_or_else(|_| panic!("Unknown rule in pattern: {}", token));
                        patterns.push(Pattern::RuleReference(sub_rule));
                    }
                }

                grammar.add_rule(rule, patterns);
            }
        }

        grammar
    }
}
