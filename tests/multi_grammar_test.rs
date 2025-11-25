use multilang::grammar::Grammar;
use std::fs;

use crate::test_utils::run_code_and_check;

mod test_utils;

fn load_grammar(path: &str) -> Grammar {
    let grammar_def = fs::read_to_string(path).expect(&format!("Failed to read grammar: {}", path));
    Grammar::parse(&grammar_def)
}

// Standard Grammar Tests
#[test]
fn test_standard_two_sum() {
    let grammar = load_grammar("tests/resources/standard/grammar.mlg");
    let mlc = fs::read_to_string("tests/resources/standard/two_sum.mlc").unwrap();
    run_code_and_check(&grammar, &mlc, "[0, 1]");
}

#[test]
fn test_standard_palindrome() {
    let grammar = load_grammar("tests/resources/standard/grammar.mlg");
    let mlc = fs::read_to_string("tests/resources/standard/palindrome.mlc").unwrap();
    run_code_and_check(&grammar, &mlc, "1\n0");
}

#[test]
fn test_standard_fizzbuzz() {
    let grammar = load_grammar("tests/resources/standard/grammar.mlg");
    let mlc = fs::read_to_string("tests/resources/standard/fizzbuzz.mlc").unwrap();
    run_code_and_check(
        &grammar,
        &mlc,
        "[1, 2, Fizz, 4, Buzz, Fizz, 7, 8, Fizz, Buzz, 11, Fizz, 13, 14, FizzBuzz]",
    );
}

// Wordy Grammar Tests
#[test]
fn test_wordy_two_sum() {
    let grammar = load_grammar("tests/resources/wordy/grammar.mlg");
    let mlc = fs::read_to_string("tests/resources/wordy/two_sum.mlc").unwrap();
    run_code_and_check(&grammar, &mlc, "[0, 1]");
}

#[test]
fn test_wordy_palindrome() {
    let grammar = load_grammar("tests/resources/wordy/grammar.mlg");
    let mlc = fs::read_to_string("tests/resources/wordy/palindrome.mlc").unwrap();
    run_code_and_check(&grammar, &mlc, "1\n0");
}

#[test]
fn test_wordy_fizzbuzz() {
    let grammar = load_grammar("tests/resources/wordy/grammar.mlg");
    let mlc = fs::read_to_string("tests/resources/wordy/fizzbuzz.mlc").unwrap();
    run_code_and_check(
        &grammar,
        &mlc,
        "[1, 2, Fizz, 4, Buzz, Fizz, 7, 8, Fizz, Buzz, 11, Fizz, 13, 14, FizzBuzz]",
    );
}

// Cryptic Grammar Tests
#[test]
fn test_cryptic_two_sum() {
    let grammar = load_grammar("tests/resources/cryptic/grammar.mlg");
    let mlc = fs::read_to_string("tests/resources/cryptic/two_sum.mlc").unwrap();
    run_code_and_check(&grammar, &mlc, "[0, 1]");
}

#[test]
fn test_cryptic_palindrome() {
    let grammar = load_grammar("tests/resources/cryptic/grammar.mlg");
    let mlc = fs::read_to_string("tests/resources/cryptic/palindrome.mlc").unwrap();
    run_code_and_check(&grammar, &mlc, "1\n0");
}

#[test]
fn test_cryptic_fizzbuzz() {
    let grammar = load_grammar("tests/resources/cryptic/grammar.mlg");
    let mlc = fs::read_to_string("tests/resources/cryptic/fizzbuzz.mlc").unwrap();
    run_code_and_check(
        &grammar,
        &mlc,
        "[1, 2, Fizz, 4, Buzz, Fizz, 7, 8, Fizz, Buzz, 11, Fizz, 13, 14, FizzBuzz]",
    );
}
