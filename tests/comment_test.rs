use multilang::grammar::Grammar;
use multilang::node::{Context, Value};
use multilang::parser::Parser;

#[test]
fn test_comments() {
    let grammar_def = r#"
        Program = Stmt*
        Stmt = Return
        Return = "return" Expr
        Expr = Int
        Int = [[0-9]+]
    "#;

    let grammar = Grammar::parse(grammar_def);

    let code = r#"
        // This is a comment
        return 10 // Inline comment
        // Another comment
    "#;

    let parser = Parser::new(&grammar, code);
    let node = parser.parse("Program").expect("Failed to parse comments");
    let mut ctx = Context::new();
    let result = node.run(&mut ctx).expect("Runtime error");
    assert_eq!(result, Value::Int(10));
}
