use multilang::grammar::{Grammar, Rule};
use multilang::node::{Context, Value};
use multilang::parser::Parser;

fn get_grammar() -> Grammar {
    let grammar_def = r##"
        Program = Stmt*
        Stmt = FunctionDef | FunctionCall | Return | Assignment | ForLoop | WhileLoop
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
    "##;
    Grammar::parse(grammar_def)
}

#[test]
fn test_while_loop() {
    let code = "
        i = 0
        sum = 0
        while i < 5 {
            sum = sum + i
            i = i + 1
        }
    ";
    let grammar = get_grammar();
    let parser = Parser::new(&grammar, code);
    let node = parser.parse(Rule::Program).expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx).expect("Failed to run");

    let sum = ctx.variables.get("sum").expect("Variable sum not found");
    assert_eq!(*sum, Value::Int(10)); // 0+1+2+3+4 = 10
}

#[test]
fn test_for_loop_literal() {
    let code = "
        sum = 0
        for x in [1, 2, 3] {
            sum = sum + x
        }
    ";
    let grammar = get_grammar();
    let parser = Parser::new(&grammar, code);
    let node = parser.parse(Rule::Program).expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx).expect("Failed to run");

    let sum = ctx.variables.get("sum").expect("Variable sum not found");
    assert_eq!(*sum, Value::Int(6));
}

#[test]
fn test_for_loop_variable() {
    let code = "
        list = [10, 20, 30]
        sum = 0
        for x in list {
            sum = sum + x
        }
    ";
    let grammar = get_grammar();
    let parser = Parser::new(&grammar, code);
    let node = parser.parse(Rule::Program).expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx).expect("Failed to run");

    let sum = ctx.variables.get("sum").expect("Variable sum not found");
    assert_eq!(*sum, Value::Int(60));
}

#[test]
fn test_nested_loops() {
    let code = "
        sum = 0
        for i in [1, 2] {
            for j in [10, 20] {
                sum = sum + i + j
            }
        }
    ";
    // i=1: j=10 -> sum=11, j=20 -> sum=11+21=32
    // i=2: j=10 -> sum=32+12=44, j=20 -> sum=44+22=66

    let grammar = get_grammar();
    let parser = Parser::new(&grammar, code);
    let node = parser.parse(Rule::Program).expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx).expect("Failed to run");

    let sum = ctx.variables.get("sum").expect("Variable sum not found");
    assert_eq!(*sum, Value::Int(66));
}
