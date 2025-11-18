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
fn test_simple_print() {
    let grammar = r#"
        Program = Stmt*
        Stmt = Print
        Print = "print" Int
        Int = [[0-9]+]
    "#;
    let input = "print 123";
    test_script(grammar, input, Value::Void);
}
