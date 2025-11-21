use multilang::grammar::Grammar;
use multilang::node::{Context, Value};
use multilang::parser::Parser;

fn run_code_with_time(code: &str) -> Value {
    let grammar_def = r#"
        Program = Stmt*
        Stmt = FunctionDef
        Stmt = FunctionCall
        Stmt = Return
        Stmt = If
        
        FunctionDef = "fn" name:Identifier "(" params:ParamList ")" "{" body:Block "}"
        ParamList = name:Identifier "," params:ParamList
        ParamList = name:Identifier
        
        Block = Program
        
        FunctionCall = name:Identifier "(" args:ArgList ")"
        FunctionCall = name:Identifier "(" ")"
        ArgList = Expr "," args:ArgList
        ArgList = Expr
        
        Return = "return" Expr
        
        If = IfElse | IfThen
        IfElse = "if" condition:Expr "{" then:Block "}" "else" "{" else:Block "}"
        IfThen = "if" condition:Expr "{" then:Block "}"
        
        Expr = LogicalOr
        LogicalOr = LogicalAnd "||" LogicalOr | LogicalAnd
        LogicalAnd = Comparison "&&" LogicalAnd | Comparison
        Comparison = Term Eq Term | Term Neq Term | Term Lt Term | Term Gt Term | Term
        Term = Factor Add Term | Factor Sub Term | Factor
        Factor = Unary Mul Factor | Unary Div Factor | Unary
        Unary = UnaryOp Unary | Atom
        Atom = Float | Int | String | FunctionCall | Identifier | "(" Expr ")"
        
        UnaryOp = [!]
        Eq = [==]
        Neq = [!=]
        Lt = [<]
        Gt = [>]
        Add = [\+]
        Sub = [-]
        Mul = [\*]
        Div = [/]
        
        Float = [[0-9]+\.[0-9]+]
        Int = [[0-9]+]
        String = ["[^\"]*"]
        Identifier = [[a-zA-Z_][a-zA-Z0-9_]*]
    "#;

    let start_grammar = std::time::Instant::now();
    let grammar = Grammar::parse(grammar_def);
    println!("Grammar parse time: {:?}", start_grammar.elapsed());

    let start_parse = std::time::Instant::now();
    let parser = Parser::new(&grammar, code);
    let node = parser.parse("Program").expect("Failed to parse");
    println!("Code parse time: {:?}", start_parse.elapsed());

    let start_run = std::time::Instant::now();
    let mut ctx = Context::new();
    let res = node.run(&mut ctx);
    println!("Execution time: {:?}", start_run.elapsed());

    res
}

#[test]
fn test_fibonacci_recursive_small() {
    let code = r#"
        fn fib(n) {
            if n < 2 {
                return n
            } else {
                return fib(n - 1) + fib(n - 2)
            }
        }
        
        return fib(10)
    "#;

    println!("Running recursive Fibonacci(10)...");
    let result = run_code_with_time(code);
    assert_eq!(result, Value::Int(55));
}

#[test]
fn test_fibonacci_recursive_medium() {
    let code = r#"
        fn fib(n) {
            if n < 2 {
                return n
            } else {
                return fib(n - 1) + fib(n - 2)
            }
        }
        
        return fib(20)
    "#;

    println!("Running recursive Fibonacci(20)...");
    let result = run_code_with_time(code);
    assert_eq!(result, Value::Int(6765));
}

#[test]
fn test_fibonacci_recursive_large() {
    // fib(20) = 6765
    // This will stress the interpreter more
    let code = r#"
        fn fib(n) {
            if n < 2 {
                return n
            } else {
                return fib(n - 1) + fib(n - 2)
            }
        }
        
        return fib(30)
    "#;

    println!("Running recursive Fibonacci(30)...");
    let result = run_code_with_time(code);
    assert_eq!(result, Value::Int(832040));
}
