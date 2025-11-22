use multilang::grammar::Grammar;
use multilang::node::{Context, Value};
use multilang::parser::Parser;
use std::cell::RefCell;
use std::rc::Rc;

fn test_script(grammar_def: &str, input: &str, expected: Value) {
    let grammar = Grammar::parse(grammar_def);
    let parser = Parser::new(&grammar, input);
    let program_node = parser.parse("Program").expect("Parsing failed");
    let mut ctx = Context::new();
    let result = program_node.run(&mut ctx).expect("Runtime error");
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
    node.run(&mut ctx).expect("Runtime error")
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
        assert_eq!(*val.borrow(), "hello world");
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
        assert_eq!(*val.borrow(), "hello world");
    } else {
        panic!("Expected String, got {:?}", result);
    }
}

#[test]
fn test_negative_numbers() {
    // We need to update the grammar in run_code to support UnaryOp for negative numbers
    // But run_code uses a hardcoded grammar.
    // So I will define a new grammar here that includes UnaryOp and test it.
    // Actually, the user asked to "add a test... If they are not implement them".
    // So I should try to use the "standard" grammar if possible, but run_code's grammar is local.
    // I will update run_code's grammar to match main.rs more closely, including UnaryOp.

    let grammar_def = r#"
        Program = Stmt*
        Stmt = Return
        Return = "return" Expr
        
        Expr = Term
        Term = Factor Add Term | Factor Sub Term | Factor
        Factor = Unary Mul Factor | Unary Div Factor | Unary
        Unary = UnaryOp Unary | Atom
        Atom = Float | Int | "(" Expr ")"
        
        UnaryOp = [-]
        Add = [\+]
        Sub = [-]
        Mul = [\*]
        Div = [/]
        
        Float = [[0-9]+\.[0-9]+]
        Int = [[0-9]+]
    "#;

    let grammar = Grammar::parse(grammar_def);

    // Test negative int
    let code = "return -5";
    let parser = Parser::new(&grammar, code);
    let node = parser
        .parse("Program")
        .expect("Failed to parse negative int");
    let mut ctx = Context::new();
    let result = node.run(&mut ctx).expect("Runtime error");
    assert_eq!(result, Value::Int(-5));

    // Test negative float
    let code = "return -3.14";
    let parser = Parser::new(&grammar, code);
    let node = parser
        .parse("Program")
        .expect("Failed to parse negative float");
    let mut ctx = Context::new();
    let result = node.run(&mut ctx).expect("Runtime error");
    if let Value::Float(val) = result {
        assert!((val - -3.14).abs() < 1e-6);
    } else {
        panic!("Expected Float, got {:?}", result);
    }

    // Test arithmetic with negative
    let code = "return 5 + -3";
    let parser = Parser::new(&grammar, code);
    let node = parser
        .parse("Program")
        .expect("Failed to parse arithmetic with negative");
    let mut ctx = Context::new();
    let result = node.run(&mut ctx).expect("Runtime error");
    assert_eq!(result, Value::Int(2));
}

#[test]
fn test_modulo() {
    let grammar_def = r#"
        Program = Stmt*
        Stmt = Return
        Return = "return" Expr
        
        Expr = Term
        Term = Factor Add Term | Factor Sub Term | Factor
        Factor = Unary Mul Factor | Unary Div Factor | Unary Mod Factor | Unary
        Unary = UnaryOp Unary | Atom
        Atom = Float | Int
        
        UnaryOp = [!] | [-]
        Add = [\+]
        Sub = [-]
        Mul = [\*]
        Div = [/]
        Mod = [%]
        
        Float = [[0-9]+\.[0-9]+]
        Int = [[0-9]+]
    "#;

    let grammar = Grammar::parse(grammar_def);

    let code = "return 10 % 3";
    let parser = Parser::new(&grammar, code);
    let node = parser.parse("Program").expect("Failed to parse modulo");
    let mut ctx = Context::new();
    let result = node.run(&mut ctx).expect("Runtime error");
    assert_eq!(result, Value::Int(1));
}
