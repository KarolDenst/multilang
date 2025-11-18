use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(String),
    RuleReference(String),
    Regex(String),
    Star(Box<Pattern>),
    Named(String, Box<Pattern>), // name:Pattern
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub patterns: Vec<Pattern>,
}

#[derive(Debug, Clone)]
pub struct Grammar {
    pub rules: HashMap<String, Vec<Rule>>,
}

impl Grammar {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, name: &str, patterns: Vec<Pattern>) {
        self.rules
            .entry(name.to_string())
            .or_insert_with(Vec::new)
            .push(Rule {
                name: name.to_string(),
                patterns,
            });
    }

    pub fn parse(input: &str) -> Self {
        let mut grammar = Grammar::new();
        
        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() != 2 {
                continue;
            }

            let name = parts[0].trim();
            let rhs = parts[1].trim().trim_end_matches(';');
            
            for alternative in rhs.split('|') {
                let mut patterns = Vec::new();
                for token in alternative.split_whitespace() {
                    if token.starts_with('"') && token.ends_with('"') {
                        patterns.push(Pattern::Literal(token[1..token.len()-1].to_string()));
                    } else if token.starts_with('[') && token.ends_with(']') {
                        patterns.push(Pattern::Regex(token[1..token.len()-1].to_string()));
                    } else if token.ends_with('*') {
                         let sub_token = &token[..token.len()-1];
                         patterns.push(Pattern::Star(Box::new(Pattern::RuleReference(sub_token.to_string()))));
                    } else if let Some(idx) = token.find(':') {
                        // Handle name:Pattern
                        let field_name = &token[..idx];
                        let sub_token = &token[idx+1..];
                        
                        let sub_pattern = if sub_token.starts_with('"') && sub_token.ends_with('"') {
                            Pattern::Literal(sub_token[1..sub_token.len()-1].to_string())
                        } else if sub_token.starts_with('[') && sub_token.ends_with(']') {
                            Pattern::Regex(sub_token[1..sub_token.len()-1].to_string())
                        } else {
                            Pattern::RuleReference(sub_token.to_string())
                        };
                        
                        patterns.push(Pattern::Named(field_name.to_string(), Box::new(sub_pattern)));
                    } else {
                        patterns.push(Pattern::RuleReference(token.to_string()));
                    }
                }
                
                grammar.add_rule(name, patterns);
            }
        }

        grammar
    }
}
