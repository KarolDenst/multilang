use multilang::grammar::Grammar;
use multilang::parser::Parser;
use multilang::node::{Context, Value};

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
        Comparison = Term CompOp Term | Term
        Term = Factor AddOp Term | Factor
        Factor = Atom MulOp Factor | Atom
        Atom = Float | Int | String | Identifier | FunctionCall | "(" Expr ")"
        
        CompOp = [==] | [!=] | [<] | [>]
        AddOp = [\+] | [-]
        MulOp = [\*] | [/]
        
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
