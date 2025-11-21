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
        Expr = Term
        Term = Factor Add Term | Factor Sub Term | Factor
        Factor = Int Mul Factor | Int Div Factor | Int
        
        Add = [\+]
        Sub = [-]
        Mul = [\*]
        Div = [/]
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
        Expr = Term
        
        Term = Factor Add Term | Factor Sub Term | Factor
        Factor = Int Mul Factor | Int Div Factor | Int
        
        Add = [\+]
        Sub = [-]
        Mul = [\*]
        Div = [/]
        Int = [[0-9]+]
    "#;
    
    // 2 + 3 * 4 = 14 (not 20)
    test_script(grammar, "2 + 3 * 4", Value::Int(14));
    // 2 * 3 + 4 = 10
    test_script(grammar, "2 * 3 + 4", Value::Int(10));
}



fn run_code(code: &str) -> Value {
    let grammar_def = r#"
        Program = Stmt*
        Stmt = FunctionDef
        Stmt = FunctionCall
        Stmt = Return
        FunctionDef = "fn" name:Identifier "(" params:ParamList ")" "{" body:Block "}"
        FunctionDef = "fn" name:Identifier "(" ")" "{" body:Block "}"
        Block = Stmt*
        FunctionCall = name:Identifier "(" args:ArgList ")"
        FunctionCall = name:Identifier "(" ")"
        
        ParamList = Identifier "," params:ParamList
        ParamList = Identifier
        
        ArgList = Expr
        
        Return = "return" Expr
        
        Expr = Term
        Term = Factor Add Term | Factor Sub Term | Factor
        Factor = Atom Mul Factor | Atom Div Factor | Atom
        Atom = Float | Int | String | Identifier | FunctionCall | "(" Expr ")"
        
        Add = [\+]
        Sub = [-]
        Mul = [\*]
        Div = [/]
        
        Float = [[0-9]+\.[0-9]+]
        Int = [[0-9]+]
        String = ["[^\"]*"]
        Identifier = [[a-zA-Z_][a-zA-Z0-9_]*]
    "#;

    let grammar = Grammar::parse(grammar_def);
    let parser = Parser::new(&grammar, code);
    let node = parser.parse("Program").expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx)
}

#[test]
fn test_float_literal() {
    let code = r#"
        return 3.14
    "#;
    let result = run_code(code);
    if let Value::Float(val) = result {
        assert!((val - 3.14).abs() < 1e-6);
    } else {
        panic!("Expected Float, got {:?}", result);
    }
}

#[test]
fn test_string_literal() {
    let code = r#"
        return "hello world"
    "#;
    let result = run_code(code);
    if let Value::String(val) = result {
        assert_eq!(val, "hello world");
    } else {
        panic!("Expected String, got {:?}", result);
    }
}

#[test]
fn test_float_arithmetic() {
    let code = r#"
        return 1.5 + 2.5
    "#;
    let result = run_code(code);
    if let Value::Float(val) = result {
        assert!((val - 4.0).abs() < 1e-6);
    } else {
        panic!("Expected Float, got {:?}", result);
    }
}

#[test]
fn test_string_concatenation() {
    let code = r#"
        return "hello" + " " + "world"
    "#;
    let result = run_code(code);
    if let Value::String(val) = result {
        assert_eq!(val, "hello world");
    } else {
        panic!("Expected String, got {:?}", result);
    }
}
