use multilang::grammar::Grammar;
use multilang::node::Context;
use multilang::parser::Parser;

fn main() {
    // 1. Define Grammar
    let grammar_def = r##"
        Program = Stmt*
        Stmt = FunctionDef | FunctionCall | Print | Return | Assignment | ForLoop | WhileLoop
        FunctionDef = "fn" name:Identifier "(" params:ParamList ")" "{" body:Block "}"
        FunctionDef = "fn" name:Identifier "(" ")" "{" body:Block "}"
        Block = Stmt*
        FunctionCall = name:Identifier "(" args:ArgList ")"
        FunctionCall = name:Identifier "(" ")"
        
        Assignment = name:Identifier "=" value:Expr
        
        ForLoop = "for" variable:Identifier "in" iterable:Expr "{" body:Block "}"
        WhileLoop = "while" condition:Expr "{" body:Block "}"
        
        ParamList = Identifier "," params:ParamList
        ParamList = Identifier
        
        ArgList = Expr "," args:ArgList
        ArgList = Expr
        
        Return = "return" value:Expr
        
        Expr = LogicalOr
        LogicalOr = LogicalAnd "||" LogicalOr | LogicalAnd
        LogicalAnd = Comparison "&&" LogicalAnd | Comparison
        Comparison = Term Eq Term | Term Neq Term | Term Lt Term | Term Gt Term | Term
        Term = Factor Add Term | Factor Sub Term | Factor
        Factor = Unary Mul Factor | Unary Div Factor | Unary
        Unary = UnaryOp Unary | Atom
        Atom = Float | Int | String | FunctionCall | Identifier | ListLiteral | "(" Expr ")"
        
        ListLiteral = "[" Elements "]"
        ListLiteral = "[" "]"
        
        Elements = Expr "," Elements
        Elements = Expr
        
        UnaryOp = [!] | [-]
        Eq = [==]
        Neq = [!=]
        Lt = [<]
        Gt = [>]
        Add = [+]
        Sub = [-]
        Mul = [*]
        Div = [/]
        
        Float = [[0-9]+[.][0-9]+]
        Int = [[0-9]+]
        String = ["[^\"]*"]
        Identifier = [[a-zA-Z_][a-zA-Z0-9_]*]
    "##;

    println!("Parsing grammar...");
    let grammar = Grammar::parse(grammar_def);

    // 2. Define Input Script
    let input = r#"
        fn add(a, b) {
            return a
        }
        print(100)
        add(10, 20)
    "#;

    // 3. Parse Input
    println!("Parsing input: '{}'", input);
    let parser = Parser::new(&grammar, input);
    match parser.parse("Program") {
        Ok(program_node) => {
            println!("Parsing successful! Running program...");
            // 4. Run Program
            let mut ctx = Context::new();

            match program_node.run(&mut ctx) {
                Ok(result) => {
                    println!("Program returned: {:?}", result);

                    if let multilang::node::Value::Int(val) = result {
                        assert_eq!(val, 10);
                        println!("Assertion passed: Returned 10");
                    } else {
                        println!("Assertion failed: Expected Int(10), got {:?}", result);
                    }
                }
                Err(e) => {
                    println!("Runtime Error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Parsing failed: {}", e);
        }
    }
}
