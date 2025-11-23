use multilang::grammar::{Grammar, Rule};
use multilang::parser::Parser;

#[test]
fn test_keyword_fn_in_sequence_works() {
    // This test shows that keywords in sequences work fine
    let grammar_def = r##"
        Program = Stmt*
        Stmt = FunctionDef
        
        FunctionDef = "fn" name:Identifier "(" ")" "{" body:Block "}"
        
        Block = Stmt*
        
        Identifier = [[a-zA-Z_][a-zA-Z0-9_]*]
    "##;

    let grammar = Grammar::parse(grammar_def);
    let code = "fn test() {}";
    let parser = Parser::new(&grammar, code);
    let result = parser.parse(Rule::Program);

    assert!(
        result.is_ok(),
        "Keywords in sequences should work: {:?}",
        result.err()
    );
}

#[test]
fn test_function_keyword_in_expression_context() {
    // Verify function keyword cannot be used as variable name
    let grammar_def = r##"
        Program = Stmt*
        Stmt = FunctionDef | Assignment
        
        FunctionDef = "function" name:Identifier "(" ")" "{" body:Block "}"
        Assignment = name:Identifier "=" value:Identifier
        
        Block = Stmt*
        
        Identifier = [[a-zA-Z_][a-zA-Z0-9_]*]
    "##;

    let grammar = Grammar::parse(grammar_def);

    // This should now fail parsing because "function" is a keyword
    let code = "x = function";
    let parser = Parser::new(&grammar, code);
    let result = parser.parse(Rule::Program);

    // Should now FAIL
    assert!(
        result.is_err(),
        "SUCCESS: 'function' cannot be used as variable name!"
    );
}

#[test]
#[should_panic]
fn test_def_keyword_should_fail_as_identifier() {
    let grammar_def = r##"
        Program = Stmt*
        Stmt = FunctionDef | VarRef
        
        FunctionDef = "def" name:Identifier "(" ")" "{" body:Block "}"
        
        Block = Stmt*
        
        VarRef = Identifier
        
        Identifier = [[a-zA-Z_][a-zA-Z0-9_]*]
    "##;

    let grammar = Grammar::parse(grammar_def);

    // "def" alone should NOT parse as Identifier
    let code = "def";
    let parser = Parser::new(&grammar, code);
    let result = parser.parse(Rule::Program);

    // This should fail but currently passes
    assert!(
        result.is_err(),
        "'def' should not be accepted as identifier"
    );
}
