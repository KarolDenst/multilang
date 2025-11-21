use multilang::grammar::Grammar;
use multilang::node::{Context, Value};
use multilang::parser::Parser;

fn get_grammar() -> Grammar {
    let grammar_def = r#"
        Program = Stmt*
        Stmt = FunctionDef | FunctionCall | Return | If | Assignment
        FunctionDef = "fn" name:Identifier "(" params:ParamList ")" "{" body:Block "}"
        FunctionDef = "fn" name:Identifier "(" ")" "{" body:Block "}"
        Block = Stmt*
        FunctionCall = name:Identifier "(" args:ArgList ")"
        FunctionCall = name:Identifier "(" ")"
        
        Assignment = name:Identifier "=" value:Expr
        
        ParamList = Identifier "," params:ParamList
        ParamList = Identifier
        
        ArgList = Expr "," args:ArgList
        ArgList = Expr
        
        Return = "return" value:Expr
        
        If = "if" condition:Expr "{" then:Block "}"
        
        Expr = Term
        Term = Factor Add Term | Factor Sub Term | Factor
        Factor = Atom Mul Factor | Atom Div Factor | Atom
        Atom = Float | Int | String | FunctionCall | Identifier | "(" Expr ")"
        
        Add = [\+]
        Sub = [-]
        Mul = [\*]
        Div = [/]
        
        Float = [[0-9]+\.[0-9]+]
        Int = [[0-9]+]
        String = ["[^\"]*"]
        Identifier = [[a-zA-Z_][a-zA-Z0-9_]*]
    "#;
    Grammar::parse(grammar_def)
}

#[test]
fn test_assignment() {
    let grammar = get_grammar();
    let code = r#"
        x = 10
        return x
    "#;
    let parser = Parser::new(&grammar, code);
    let node = parser.parse("Program").expect("Parsing failed");
    let mut ctx = Context::new();
    let result = node.run(&mut ctx).expect("Runtime error");
    assert_eq!(result, Value::Int(10));
}

#[test]
fn test_assignment_update() {
    let grammar = get_grammar();
    let code = r#"
        x = 10
        x = 20
        return x
    "#;
    let parser = Parser::new(&grammar, code);
    let node = parser.parse("Program").expect("Parsing failed");
    let mut ctx = Context::new();
    let result = node.run(&mut ctx).expect("Runtime error");
    assert_eq!(result, Value::Int(20));
}

#[test]
fn test_assignment_expression() {
    let grammar = get_grammar();
    let code = r#"
        x = 10 + 5 * 2
        return x
    "#;
    let parser = Parser::new(&grammar, code);
    let node = parser.parse("Program").expect("Parsing failed");
    let mut ctx = Context::new();
    let result = node.run(&mut ctx).expect("Runtime error");
    assert_eq!(result, Value::Int(20));
}

#[test]
fn test_assignment_in_function() {
    let grammar = get_grammar();
    let code = r#"
        fn foo() {
            x = 100
            return x
        }
        return foo()
    "#;
    let parser = Parser::new(&grammar, code);
    let node = parser.parse("Program").expect("Parsing failed");
    let mut ctx = Context::new();
    let result = node.run(&mut ctx).expect("Runtime error");
    assert_eq!(result, Value::Int(100));
}
