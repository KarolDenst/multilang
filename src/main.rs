use multilang::grammar::Grammar;
use multilang::parser::Parser;
use multilang::node::Context;

fn main() {
    // 1. Define Grammar
    let grammar_def = r#"
        Program = Stmt*
        Stmt = Print
        Stmt = Return
        Print = "print" Int
        Return = "return" Int
        Int = [[0-9]+]
    "#;

    println!("Parsing grammar...");
    let grammar = Grammar::parse(grammar_def);
    // println!("Grammar rules: {:?}", grammar.rules.keys());

    // 2. Define Input Script
    let input = "print 42 return 100";

    // 3. Parse Input
    println!("Parsing input: '{}'", input);
    let parser = Parser::new(&grammar, input);
    match parser.parse("Program") {
        Ok(program_node) => {
            println!("Parsing successful! Running program...");
            // 4. Run Program
            let mut ctx = Context::new();
            let result = program_node.run(&mut ctx);
            println!("Program returned: {:?}", result);
            
            if let multilang::node::Value::Int(val) = result {
                assert_eq!(val, 100);
                println!("Assertion passed: Returned 100");
            } else {
                println!("Assertion failed: Expected Int(100), got {:?}", result);
            }
        }
        Err(e) => {
            println!("Parsing failed: {}", e);
        }
    }
}
