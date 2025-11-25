use multilang::grammar::Grammar;
use std::fs;

use crate::test_utils::run_code_and_check;

mod test_utils;

fn load_standard_grammar() -> Grammar {
    let grammar_def = fs::read_to_string("tests/resources/standard/grammar.mlg")
        .expect("Failed to read standard grammar");
    Grammar::parse(&grammar_def)
}

#[test]
fn test_function_args() {
    let grammar = load_standard_grammar();

    // Test single argument
    let input1 = r#"
        fn identity(x) {
            return x
        }
        print(identity(42))
    "#;
    run_code_and_check(&grammar, input1, "42");

    // Test multiple arguments
    let input2 = r#"
        fn add(a, b) {
            return b
        }
        print(add(10, 20))
    "#;
    run_code_and_check(&grammar, input2, "20");
}

#[test]
fn test_nested_calls_with_args() {
    let grammar = load_standard_grammar();

    let input = r#"
        fn foo(x) {
            return x
        }
        fn bar(y) {
            foo(y)
        }
        print(bar(100))
    "#;

    run_code_and_check(&grammar, input, "100");
}
