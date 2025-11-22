use multilang::grammar::{Grammar, Rule};
use multilang::node::{Context, Value};
use multilang::parser::Parser;

fn test_script(grammar_def: &str, input: &str, expected: Value) {
    let grammar = Grammar::parse(grammar_def);
    let parser = Parser::new(&grammar, input);
    let program_node = parser.parse(Rule::Program).expect("Parsing failed");
    let mut ctx = Context::new();
    let result = program_node.run(&mut ctx).expect("Runtime error");
    assert_eq!(result, expected);
}

#[test]
fn test_return_value() {
    let grammar = r#"
        Program = Stmt*
        Stmt = Return
        Return = "return" Expr
        Expr = Term
        Term = Factor Add Term | Factor Sub Term | Factor
        Factor = Atom Mul Factor | Atom Div Factor | Atom
        Atom = Float | Int | String | Identifier | FunctionCall | "(" Expr ")"
        
        Add = [\+]
        Sub = [-]
        Mul = [\*]
        Div = [/]
        Int = [[0-9]+]
    "#;
    let input = "return 100";
    test_script(grammar, input, Value::Int(100));
}
