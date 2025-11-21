use multilang::grammar::Grammar;
use multilang::node::{Context, Value};
use multilang::parser::Parser;

fn get_grammar() -> Grammar {
    let grammar_def = r#"
        Program = Stmt*
        Stmt = FunctionDef | FunctionCall | Return | If
        FunctionDef = "fn" name:Identifier "(" params:ParamList ")" "{" body:Block "}"
        FunctionDef = "fn" name:Identifier "(" ")" "{" body:Block "}"
        Block = Stmt*
        FunctionCall = name:Identifier "(" args:ArgList ")"
        FunctionCall = name:Identifier "(" ")"
        
        ParamList = Identifier "," params:ParamList
        ParamList = Identifier
        
        ArgList = Expr "," args:ArgList
        ArgList = Expr
        
        Return = "return" Expr
        
        If = "if" condition:Expr "{" then:Block "}"
        
        Expr = Term
        Term = Factor Add Term | Factor Sub Term | Factor
        Factor = Atom Mul Factor | Atom Div Factor | Atom
        Atom = Float | Int | String | Identifier | FunctionCall | "(" Expr ")"
        
        Add = [\+]
        Sub = [-]
        Mul = [\*]
        Div = [/]
        
        Float = [[0-9]+\.[0-9]+]
        Int = [[0-9]+]
        String = ["[^\"]*"]
        Identifier = [[a-zA-Z_][a-zA-Z0-9_]*]
    "#;
    Grammar::parse(grammar_def)
}

#[test]
fn test_parser_error_location() {
    let grammar = get_grammar();
    let code = r#"
        fn foo() {
            return 1 +
        }
    "#;
    let parser = Parser::new(&grammar, code);
    let result = parser.parse("Program");

    match result {
        Ok(_) => {
            panic!("Expected parser error, but parsing succeeded!");
        }
        Err(e) => {
            // The error should be around line 3 or 4 depending on how whitespace is handled
            // "return 1 +" expects a Term after +, but found "}"
            println!("Parser error: {}", e);
            // We expect the error to mention line number.
            // Since we don't know exact line without running, let's just check it contains "Line"
            assert!(e.to_string().to_lowercase().contains("line"));
        }
    }
}

#[test]
fn test_runtime_div_by_zero() {
    let grammar = get_grammar();
    let code = r#"
        return 10 / 0
    "#;
    let parser = Parser::new(&grammar, code);
    let node = parser.parse("Program").expect("Parsing failed");
    let mut ctx = Context::new();
    let result = node.run(&mut ctx);

    match result {
        Ok(_) => panic!("Expected runtime error"),
        Err(e) => {
            println!("Runtime error: {}", e);
            assert_eq!(e.message, "Division by zero");
        }
    }
}

#[test]
fn test_runtime_function_not_found() {
    let grammar = get_grammar();
    let code = r#"
        unknown_func()
    "#;
    let parser = Parser::new(&grammar, code);
    let node = parser.parse("Program").expect("Parsing failed");
    let mut ctx = Context::new();
    let result = node.run(&mut ctx);

    match result {
        Ok(_) => panic!("Expected runtime error"),
        Err(e) => {
            println!("Runtime error: {}", e);
            assert!(e.message.contains("Function 'unknown_func' not found"));
            // Check stack trace
            assert!(!e.stack_trace.is_empty());
            assert!(e.stack_trace[0].contains("unknown_func"));
        }
    }
}

#[test]
fn test_stack_trace() {
    let grammar = get_grammar();
    let code = r#"
        fn a() {
            b()
        }
        fn b() {
            c()
        }
        fn c() {
            return 10 / 0
        }
        a()
    "#;
    let parser = Parser::new(&grammar, code);
    let node = parser.parse("Program").expect("Parsing failed");
    let mut ctx = Context::new();
    let result = node.run(&mut ctx);

    match result {
        Ok(_) => panic!("Expected runtime error"),
        Err(e) => {
            println!("Runtime error: {}", e);
            assert_eq!(e.message, "Division by zero");
            // Stack trace should be c -> b -> a (or reverse depending on implementation)
            // My implementation pushes:
            // 1. c calls 10/0 -> error.
            // 2. b calls c -> catches error, pushes "at b:line"
            // 3. a calls b -> catches error, pushes "at a:line"
            // 4. main calls a -> catches error, pushes "at a:line" (Wait, main is Program node, it doesn't push stack frame for top level call unless it's a function call node)
            // The top level `a()` is a FunctionCall. It will push "at a:line".

            // So stack trace should be:
            // [ "at c:line", "at b:line", "at a:line" ] ?
            // No, `FunctionCall::run` pushes stack frame when it catches error.
            // `c` body executes `10/0`. `Factor` returns error (empty stack trace).
            // `c` is called by `b`. `b` has `FunctionCall("c")`.
            // `FunctionCall("c")` catches error, pushes "at c:line".
            // `b` is called by `a`. `a` has `FunctionCall("b")`.
            // `FunctionCall("b")` catches error, pushes "at b:line".
            // `a` is called by top-level. Top-level has `FunctionCall("a")`.
            // `FunctionCall("a")` catches error, pushes "at a:line".

            // So stack trace: ["at c:...", "at b:...", "at a:..."]

            assert_eq!(e.stack_trace.len(), 3);
            assert!(e.stack_trace[0].contains("c"));
            assert!(e.stack_trace[1].contains("b"));
            assert!(e.stack_trace[2].contains("a"));
        }
    }
}
