use multilang::grammar::Grammar;
use multilang::node::{Context, Value};
use multilang::parser::Parser;
use std::cell::RefCell;
use std::rc::Rc;

fn run_code(code: &str) -> Value {
    let grammar_def = r#"
        Program = Stmt*
        Stmt = FunctionDef | FunctionCall | Print | Return | Assignment | ForLoop | WhileLoop
        FunctionDef = "fn" name:Identifier "(" params:ParamList ")" "{" body:Block "}"
        FunctionDef = "fn" name:Identifier "(" ")" "{" body:Block "}"
        Block = Stmt*
        FunctionCall = name:Identifier "(" args:ArgList ")"
        FunctionCall = name:Identifier "(" ")"
        
        Assignment = name:Identifier "=" value:Expr
        
        ForLoop = "for" variable:Identifier "in" iterable:Expr "{" body:Block "}"
        WhileLoop = "while" condition:Expr "{" body:Block "}"
        
        ParamList = Identifier "," params:ParamList
        ParamList = Identifier
        
        ArgList = Expr "," args:ArgList
        ArgList = Expr
        
        Return = "return" value:Expr
        
        Expr = LogicalOr
        LogicalOr = LogicalAnd "||" LogicalOr | LogicalAnd
        LogicalAnd = Comparison "&&" LogicalAnd | Comparison
        Comparison = Term Eq Term | Term Neq Term | Term Lt Term | Term Gt Term | Term
        Term = Factor Add Term | Factor Sub Term | Factor
        Factor = Unary Mul Factor | Unary Div Factor | Unary Mod Factor | Unary
        Unary = UnaryOp Unary | Atom
        Atom = Float | Int | String | FunctionCall | Identifier | ListLiteral | MapLiteral | "(" Expr ")"
        
        ListLiteral = "[" Elements "]"
        ListLiteral = "[" "]"
        
        Elements = Expr "," Elements
        Elements = Expr
        
        MapLiteral = "{" MapEntries "}"
        MapLiteral = "{" "}"
        
        MapEntries = MapEntry "," MapEntries
        MapEntries = MapEntry
        
        MapEntry = Key ":" Expr
        Key = String | Identifier
        
        UnaryOp = [!] | [-]
        Eq = [==]
        Neq = [!=]
        Lt = [<]
        Gt = [>]
        Add = [\+]
        Sub = [-]
        Mul = [\*]
        Div = [/]
        Mod = [%]
        
        Float = [[0-9]+[.][0-9]+]
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
fn test_map_creation_and_access() {
    let code = r#"
        m = { "a": 1, "b": 2 }
        return get(m, "a")
    "#;
    let result = run_code(code);
    assert_eq!(result, Value::Int(1));
}

#[test]
fn test_map_mutation() {
    let code = r#"
        m = { "a": 1 }
        set(m, "a", 10)
        return get(m, "a")
    "#;
    let result = run_code(code);
    assert_eq!(result, Value::Int(10));
}

#[test]
fn test_map_new_key() {
    let code = r#"
        m = {}
        set(m, "new_key", 100)
        return get(m, "new_key")
    "#;
    let result = run_code(code);
    assert_eq!(result, Value::Int(100));
}

#[test]
fn test_map_keys() {
    let code = r#"
        m = { "x": 1, "y": 2 }
        k = keys(m)
        return get(k, 0) // Order isn't guaranteed, but we expect a string
    "#;
    let result = run_code(code);
    if let Value::String(_) = result {
        // Pass
    } else {
        panic!("Expected string key, got {:?}", result);
    }
}

#[test]
fn test_nested_map() {
    let code = r#"
        m = { "inner": { "val": 42 } }
        inner = get(m, "inner")
        return get(inner, "val")
    "#;
    let result = run_code(code);
    assert_eq!(result, Value::Int(42));
}
