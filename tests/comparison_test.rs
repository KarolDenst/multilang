use multilang::grammar::{Grammar, Rule};
use multilang::node::{Context, Value};
use multilang::parser::Parser;

fn run_code(code: &str) -> Value {
    let grammar_def = r#"
        Program = Stmt*
        Stmt = FunctionCall
        Stmt = Return
        FunctionCall = name:Identifier "(" args:ArgList ")"
        FunctionCall = name:Identifier "(" ")"
        ArgList = Expr "," args:ArgList
        ArgList = Expr
        Return = "return" Expr
        
        Expr = Comparison
        Comparison = Term Eq Term | Term Neq Term | Term Lt Term | Term Gt Term | Term
        Term = Factor Add Term | Factor Sub Term | Factor
        Factor = Atom Mul Factor | Atom Div Factor | Atom
        Atom = Float | Int | String | Identifier | FunctionCall | "(" Expr ")"
        
        Eq = [==]
        Neq = [!=]
        Lt = [<]
        Gt = [>]
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
    let node = parser.parse(Rule::Program).expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx).expect("Runtime error")
}

#[test]
fn test_int_equality() {
    assert_eq!(run_code("return 1 == 1"), Value::Bool(true));
    assert_eq!(run_code("return 1 == 2"), Value::Bool(false));
    assert_eq!(run_code("return 1 != 2"), Value::Bool(true));
    assert_eq!(run_code("return 1 != 1"), Value::Bool(false));
}

#[test]
fn test_int_comparison() {
    assert_eq!(run_code("return 1 < 2"), Value::Bool(true));
    assert_eq!(run_code("return 2 < 1"), Value::Bool(false));
    assert_eq!(run_code("return 2 > 1"), Value::Bool(true));
    assert_eq!(run_code("return 1 > 2"), Value::Bool(false));
}

#[test]
fn test_float_comparison() {
    assert_eq!(run_code("return 1.0 == 1.0"), Value::Bool(true));
    assert_eq!(run_code("return 1.0 != 2.0"), Value::Bool(true));
    assert_eq!(run_code("return 1.0 < 2.0"), Value::Bool(true));
    assert_eq!(run_code("return 2.0 > 1.0"), Value::Bool(true));
}

#[test]
fn test_string_comparison() {
    assert_eq!(run_code("return \"a\" == \"a\""), Value::Bool(true));
    assert_eq!(run_code("return \"a\" != \"b\""), Value::Bool(true));
    assert_eq!(run_code("return \"a\" < \"b\""), Value::Bool(true));
    assert_eq!(run_code("return \"b\" > \"a\""), Value::Bool(true));
}

#[test]
fn test_precedence() {
    // 1 + 2 < 4 -> 3 < 4 -> True
    assert_eq!(run_code("return 1 + 2 < 4"), Value::Bool(true));
    // 1 + 2 == 3 -> 3 == 3 -> True
    assert_eq!(run_code("return 1 + 2 == 3"), Value::Bool(true));
}
