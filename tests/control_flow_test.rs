use multilang::grammar::Grammar;

use crate::test_utils::run_code_and_check;

mod test_utils;

#[test]
fn test_if_true() {
    let grammar_def = r#"
        Program = Stmt*
        Stmt = If | Print
        If = IfThen
        IfThen = "if" condition:Expr then:Block
        Block = "{" Program "}"
        Print = "print" "(" Int ")"
        Expr = True | False | Int
        True = "true"
        False = "false"
        Int = [[0-9]+]
    "#;
    let grammar = Grammar::parse(grammar_def);

    run_code_and_check(&grammar, "if true { print(10) }", "10");
}

#[test]
fn test_if_false() {
    let grammar_def = r#"
        Program = Stmt*
        Stmt = If | Print
        If = IfThen
        IfThen = "if" condition:Expr then:Block
        Block = "{" Program "}"
        Print = "print" "(" Int ")"
        Expr = True | False | Int
        True = "true"
        False = "false"
        Int = [[0-9]+]
    "#;
    let grammar = Grammar::parse(grammar_def);

    // if false { print(10) } -> should do nothing, no output
    // We'll test this by checking that the logs are empty
    let code = "if false { print(10) }";
    let (logs, _guard) = test_utils::capture_output();
    let parser = multilang::parser::Parser::new(&grammar, code);
    let node = parser
        .parse(multilang::grammar::Rule::Program)
        .expect("Parse error");
    let mut ctx = multilang::node::Context::new();
    node.run(&mut ctx).expect("Runtime error");
    assert_eq!(*logs.borrow(), Vec::<String>::new());
}

#[test]
fn test_if_else() {
    let grammar_def = r#"
        Program = Stmt*
        Stmt = If | Print
        If = IfElse
        IfElse = "if" condition:Expr then:Block "else" else:Block
        Block = "{" Program "}"
        Print = "print" "(" Int ")"
        Expr = True | False | Int
        True = "true"
        False = "false"
        Int = [[0-9]+]
    "#;
    let grammar = Grammar::parse(grammar_def);

    run_code_and_check(&grammar, "if true { print(10) } else { print(20) }", "10");
    run_code_and_check(&grammar, "if false { print(10) } else { print(20) }", "20");
}

#[test]
fn test_int_condition() {
    let grammar_def = r#"
        Program = Stmt*
        Stmt = If | Print
        If = IfThen
        IfThen = "if" condition:Expr then:Block
        Block = "{" Program "}"
        Print = "print" "(" Int ")"
        Expr = Int
        Int = [[0-9]+]
    "#;
    let grammar = Grammar::parse(grammar_def);

    // 1 is true
    run_code_and_check(&grammar, "if 1 { print(10) }", "10");
    // 0 is false - no output
    let code = "if 0 { print(10) }";
    let (logs, _guard) = test_utils::capture_output();
    let parser = multilang::parser::Parser::new(&grammar, code);
    let node = parser
        .parse(multilang::grammar::Rule::Program)
        .expect("Parse error");
    let mut ctx = multilang::node::Context::new();
    node.run(&mut ctx).expect("Runtime error");
    assert_eq!(*logs.borrow(), Vec::<String>::new());
}
