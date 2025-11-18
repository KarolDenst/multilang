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
fn test_if_true() {
    let grammar = r#"
        Program = Stmt*
        Stmt = If | Print
        If = IfThen
        IfThen = "if" condition:Expr then:Block
        Block = "{" Program "}"
        Print = "print" Int
        Expr = True | False | Int
        True = "true"
        False = "false"
        Int = [[0-9]+]
    "#;
    // if true { print 10 }
    // Should print 10. But test_script checks return value.
    // Print returns Void.
    // Let's use Return instead of Print to verify execution.
    
    let grammar_ret = r#"
        Program = Stmt*
        Stmt = If | Return
        If = IfThen
        IfThen = "if" condition:Expr then:Block
        Block = "{" Program "}"
        Return = "return" Int
        Expr = True | False | Int
        True = "true"
        False = "false"
        Int = [[0-9]+]
    "#;
    
    test_script(grammar_ret, "if true { return 10 }", Value::Int(10));
}

#[test]
fn test_if_false() {
    let grammar_ret = r#"
        Program = Stmt*
        Stmt = If | Return
        If = IfThen
        IfThen = "if" condition:Expr then:Block
        Block = "{" Program "}"
        Return = "return" Int
        Expr = True | False | Int
        True = "true"
        False = "false"
        Int = [[0-9]+]
    "#;
    
    // if false { return 10 } -> should do nothing, return Void (default Program result if no return)
    // Wait, Program returns the value of the last statement?
    // Program::run iterates statements. It returns the value of the last one?
    // Let's check Program implementation.
    // It returns the last value.
    // If `If` executes and condition is false, it returns Void.
    test_script(grammar_ret, "if false { return 10 }", Value::Void);
}

#[test]
fn test_if_else() {
    let grammar = r#"
        Program = Stmt*
        Stmt = If | Return
        If = IfElse
        IfElse = "if" condition:Expr then:Block "else" else:Block
        Block = "{" Program "}"
        Return = "return" Int
        Expr = True | False | Int
        True = "true"
        False = "false"
        Int = [[0-9]+]
    "#;
    
    test_script(grammar, "if true { return 10 } else { return 20 }", Value::Int(10));
    test_script(grammar, "if false { return 10 } else { return 20 }", Value::Int(20));
}

#[test]
fn test_int_condition() {
    let grammar = r#"
        Program = Stmt*
        Stmt = If | Return
        If = IfThen
        IfThen = "if" condition:Expr then:Block
        Block = "{" Program "}"
        Return = "return" Int
        Expr = Int
        Int = [[0-9]+]
    "#;
    
    // 1 is true
    test_script(grammar, "if 1 { return 10 }", Value::Int(10));
    // 0 is false
    test_script(grammar, "if 0 { return 10 }", Value::Void);
}
