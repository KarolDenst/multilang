use multilang::grammar::Grammar;
use multilang::parser::Parser;
use multilang::node::{Context, Value};

fn test_script(grammar_def: &str, input: &str, expected: Value) {
    let grammar = Grammar::parse(grammar_def);
    let parser = Parser::new(&grammar, input);
    let program_node = parser.parse("Program").expect("Parsing failed");
    let mut ctx = Context::new();
    let result = program_node.run(&mut ctx);
    assert_eq!(result, expected);
}

#[test]
fn test_arithmetic_basic() {
    // Simple right-recursive grammar for testing basic ops
    let grammar = r#"
        Program = Expr
        Expr = Add | Sub | Mul | Div | Int
        Add = Int "+" Expr
        Sub = Int "-" Expr
        Mul = Int "*" Expr
        Div = Int "/" Expr
        Int = [[0-9]+]
    "#;
    
    test_script(grammar, "1 + 2", Value::Int(3));
    test_script(grammar, "5 - 2", Value::Int(3));
    test_script(grammar, "3 * 4", Value::Int(12));
    test_script(grammar, "10 / 2", Value::Int(5));
}

#[test]
fn test_arithmetic_precedence() {
    // Grammar enforcing precedence: Add/Sub < Mul/Div < Int
    let grammar = r#"
        Program = Expr
        Expr = Add | Sub | Term
        
        Add = Term "+" Expr
        Sub = Term "-" Expr
        
        Term = Mul | Div | Factor
        
        Mul = Factor "*" Term
        Div = Factor "/" Term
        
        Factor = Int
        Int = [[0-9]+]
    "#;
    
    // 2 + 3 * 4 = 14 (not 20)
    test_script(grammar, "2 + 3 * 4", Value::Int(14));
    // 2 * 3 + 4 = 10
    test_script(grammar, "2 * 3 + 4", Value::Int(10));
}

#[test]
fn test_named_fields() {
    let grammar = r#"
        Program = Div
        Div = right:Int "\" left:Int
        Int = [[0-9]+]
    "#;
    
    // Input: "2 \ 10" -> 10 / 2 = 5
    test_script(grammar, "2 \\ 10", Value::Int(5));
}
