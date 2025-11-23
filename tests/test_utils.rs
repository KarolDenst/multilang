use std::cell::RefCell;
use std::rc::Rc;

pub struct TestLogGuard;

impl Drop for TestLogGuard {
    fn drop(&mut self) {
        multilang::functions::print::TEST_LOGS.with(|logs| {
            *logs.borrow_mut() = None;
        });
    }
}

pub fn capture_output() -> (Rc<RefCell<Vec<String>>>, TestLogGuard) {
    let logs = Rc::new(RefCell::new(Vec::new()));
    multilang::functions::print::TEST_LOGS.with(|l| {
        *l.borrow_mut() = Some(logs.clone());
    });
    (logs, TestLogGuard)
}
use multilang::grammar::{Grammar, Rule};
use multilang::node::Context;
use multilang::parser::Parser;

pub fn run_code_and_check(grammar: &Grammar, code: &str, expected: &str) {
    let parser = Parser::new(grammar, code);
    let node = parser
        .parse(Rule::Program)
        .map_err(|e| format!("Failed to parse: {}", e))
        .expect("Parse error");

    let (logs, _guard) = capture_output();
    let mut ctx = Context::new();
    node.run(&mut ctx)
        .map_err(|e| format!("Runtime error: {}", e))
        .expect("Runtime error");

    // Join all log entries into a single string and compare
    let actual = logs.borrow().join("");
    let expected_with_newline = format!("{}\n", expected);
    assert_eq!(actual, expected_with_newline);
}
