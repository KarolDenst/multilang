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
fn test_function_args() {
    let grammar = r#"
        Program = Stmt*
        Stmt = FunctionDef | FunctionCall | Return
        FunctionDef = "fn" name:Identifier "(" params:ParamList ")" "{" body:Block "}"
        FunctionDef = "fn" name:Identifier "(" ")" "{" body:Block "}"
        Block = Stmt*
        FunctionCall = name:Identifier "(" args:ArgList ")"
        FunctionCall = name:Identifier "(" ")"
        
        ParamList = Identifier "," params:ParamList
        ParamList = Identifier
        
        ArgList = Expr "," args:ArgList
        ArgList = Expr
        
        Return = "return" Expr
        Expr = Int | Identifier | FunctionCall
        Int = [[0-9]+]
        Identifier = [[a-zA-Z_][a-zA-Z0-9_]*]
    "#;
    
    // Test single argument
    let input1 = r#"
        fn identity(x) {
            return x
        }
        identity(42)
    "#;
    test_script(grammar, input1, Value::Int(42));
    
    // Test multiple arguments
    let input2 = r#"
        fn add(a, b) {
            return b
        }
        add(10, 20)
    "#;
    test_script(grammar, input2, Value::Int(20));
}

#[test]
fn test_nested_calls_with_args() {
    let grammar = r#"
        Program = Stmt*
        Stmt = FunctionDef | FunctionCall | Return
        FunctionDef = "fn" name:Identifier "(" params:ParamList ")" "{" body:Block "}"
        FunctionDef = "fn" name:Identifier "(" ")" "{" body:Block "}"
        Block = Stmt*
        FunctionCall = name:Identifier "(" args:ArgList ")"
        FunctionCall = name:Identifier "(" ")"
        
        ParamList = Identifier "," params:ParamList
        ParamList = Identifier
        
        ArgList = Expr "," args:ArgList
        ArgList = Expr
        
        Return = "return" Expr
        Expr = Int | Identifier | FunctionCall
        Int = [[0-9]+]
        Identifier = [[a-zA-Z_][a-zA-Z0-9_]*]
    "#;
    
    let input = r#"
        fn foo(x) {
            return x
        }
        fn bar(y) {
            foo(y)
        }
        bar(100)
    "#;
    
    test_script(grammar, input, Value::Int(100));
}
