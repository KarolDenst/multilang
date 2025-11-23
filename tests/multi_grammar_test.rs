use multilang::grammar::{Grammar, Rule};
use multilang::node::{Context, Value};
use multilang::parser::Parser;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

fn load_grammar(path: &str) -> Grammar {
    let grammar_def = fs::read_to_string(path).expect(&format!("Failed to read grammar: {}", path));
    Grammar::parse(&grammar_def)
}

fn run_mlc_with_grammar(grammar: &Grammar, mlc: &str) -> Context {
    let parser = Parser::new(grammar, mlc);
    let node = parser.parse(Rule::Program).expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx).expect("Failed to run");
    ctx
}

// Standard Grammar Tests
#[test]
fn test_standard_two_sum() {
    let grammar = load_grammar("tests/resources/standard/grammar.mlg");
    let mlc = fs::read_to_string("tests/resources/standard/two_sum.mlc").unwrap();
    let ctx = run_mlc_with_grammar(&grammar, &mlc);

    let result = ctx
        .variables
        .get("result")
        .expect("result variable not found");
    if let Value::List(l) = result {
        let list = l.borrow();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0], Value::Int(0));
        assert_eq!(list[1], Value::Int(1));
    } else {
        panic!("Expected list result");
    }
}

#[test]
fn test_standard_palindrome() {
    let grammar = load_grammar("tests/resources/standard/grammar.mlg");
    let mlc = fs::read_to_string("tests/resources/standard/palindrome.mlc").unwrap();
    let ctx = run_mlc_with_grammar(&grammar, &mlc);

    assert_eq!(ctx.variables.get("result1"), Some(&Value::Int(1))); // 121 is palindrome
    assert_eq!(ctx.variables.get("result2"), Some(&Value::Int(0))); // 123 is not
}

#[test]
fn test_standard_fizzbuzz() {
    let grammar = load_grammar("tests/resources/standard/grammar.mlg");
    let mlc = fs::read_to_string("tests/resources/standard/fizzbuzz.mlc").unwrap();
    let ctx = run_mlc_with_grammar(&grammar, &mlc);

    let result = ctx
        .variables
        .get("result")
        .expect("result variable not found");
    if let Value::List(l) = result {
        let list = l.borrow();
        assert_eq!(list.len(), 15);

        // Check a few key values
        assert_eq!(list[0], Value::Int(1));
        assert_eq!(
            list[2],
            Value::String(Rc::new(RefCell::new("Fizz".to_string())))
        );
        assert_eq!(
            list[4],
            Value::String(Rc::new(RefCell::new("Buzz".to_string())))
        );
        assert_eq!(
            list[14],
            Value::String(Rc::new(RefCell::new("FizzBuzz".to_string())))
        );
    } else {
        panic!("Expected list result");
    }
}

// Wordy Grammar Tests
#[test]
fn test_wordy_two_sum() {
    let grammar = load_grammar("tests/resources/wordy/grammar.mlg");
    let mlc = fs::read_to_string("tests/resources/wordy/two_sum.mlc").unwrap();
    let ctx = run_mlc_with_grammar(&grammar, &mlc);

    let result = ctx
        .variables
        .get("result")
        .expect("result variable not found");
    if let Value::List(l) = result {
        let list = l.borrow();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0], Value::Int(0));
        assert_eq!(list[1], Value::Int(1));
    } else {
        panic!("Expected list result");
    }
}

#[test]
fn test_wordy_palindrome() {
    let grammar = load_grammar("tests/resources/wordy/grammar.mlg");
    let mlc = fs::read_to_string("tests/resources/wordy/palindrome.mlc").unwrap();
    let ctx = run_mlc_with_grammar(&grammar, &mlc);

    assert_eq!(ctx.variables.get("result1"), Some(&Value::Int(1))); // 121 is palindrome
    assert_eq!(ctx.variables.get("result2"), Some(&Value::Int(0))); // 123 is not
}

#[test]
fn test_wordy_fizzbuzz() {
    let grammar = load_grammar("tests/resources/wordy/grammar.mlg");
    let mlc = fs::read_to_string("tests/resources/wordy/fizzbuzz.mlc").unwrap();
    let ctx = run_mlc_with_grammar(&grammar, &mlc);

    let result = ctx
        .variables
        .get("result")
        .expect("result variable not found");
    if let Value::List(l) = result {
        let list = l.borrow();
        assert_eq!(list.len(), 15);

        // Check a few key values
        assert_eq!(list[0], Value::Int(1));
        assert_eq!(
            list[2],
            Value::String(Rc::new(RefCell::new("Fizz".to_string())))
        );
        assert_eq!(
            list[4],
            Value::String(Rc::new(RefCell::new("Buzz".to_string())))
        );
        assert_eq!(
            list[14],
            Value::String(Rc::new(RefCell::new("FizzBuzz".to_string())))
        );
    } else {
        panic!("Expected list result");
    }
}

// Cryptic Grammar Tests
#[test]
fn test_cryptic_two_sum() {
    let grammar = load_grammar("tests/resources/cryptic/grammar.mlg");
    let mlc = fs::read_to_string("tests/resources/cryptic/two_sum.mlc").unwrap();
    let ctx = run_mlc_with_grammar(&grammar, &mlc);

    let result = ctx
        .variables
        .get("result")
        .expect("result variable not found");
    if let Value::List(l) = result {
        let list = l.borrow();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0], Value::Int(0));
        assert_eq!(list[1], Value::Int(1));
    } else {
        panic!("Expected list result");
    }
}

#[test]
fn test_cryptic_palindrome() {
    let grammar = load_grammar("tests/resources/cryptic/grammar.mlg");
    let mlc = fs::read_to_string("tests/resources/cryptic/palindrome.mlc").unwrap();
    let ctx = run_mlc_with_grammar(&grammar, &mlc);

    assert_eq!(ctx.variables.get("result1"), Some(&Value::Int(1))); // 121 is palindrome
    assert_eq!(ctx.variables.get("result2"), Some(&Value::Int(0))); // 123 is not
}

#[test]
fn test_cryptic_fizzbuzz() {
    let grammar = load_grammar("tests/resources/cryptic/grammar.mlg");
    let mlc = fs::read_to_string("tests/resources/cryptic/fizzbuzz.mlc").unwrap();
    let ctx = run_mlc_with_grammar(&grammar, &mlc);

    let result = ctx
        .variables
        .get("result")
        .expect("result variable not found");
    if let Value::List(l) = result {
        let list = l.borrow();
        assert_eq!(list.len(), 15);

        // Check a few key values
        assert_eq!(list[0], Value::Int(1));
        assert_eq!(
            list[2],
            Value::String(Rc::new(RefCell::new("Fizz".to_string())))
        );
        assert_eq!(
            list[4],
            Value::String(Rc::new(RefCell::new("Buzz".to_string())))
        );
        assert_eq!(
            list[14],
            Value::String(Rc::new(RefCell::new("FizzBuzz".to_string())))
        );
    } else {
        panic!("Expected list result");
    }
}
