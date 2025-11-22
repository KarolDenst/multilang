use multilang::grammar::{Grammar, Rule};
use multilang::node::{Context, Value};
use multilang::parser::Parser;

fn run_code(code: &str) -> Value {
    let grammar_def = r#"
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
        Factor = Unary Mul Factor | Unary Div Factor | Unary Mod Factor | Unary
        Unary = UnaryOp Unary | Atom
        Atom = Float | Int | String | FunctionCall | Identifier | ListLiteral | MapLiteral | "(" Expr ")"
        
        ListLiteral = "[" Elements "]"
        ListLiteral = "[" "]"
        
        Elements = Expr "," Elements
        Elements = Expr
        
        MapLiteral = "{" MapEntries "}"
        MapLiteral = "{" "}"
        
        MapEntries = MapEntry "," MapEntries
        MapEntries = MapEntry
        
        MapEntry = Key ":" Expr
        Key = String | Identifier
        
        UnaryOp = [!] | [-]
        Eq = [==]
        Neq = [!=]
        Lt = [<]
        Gt = [>]
        Add = [\+]
        Sub = [-]
        Mul = [\*]
        Div = [/]
        Mod = [%]
        
        Float = [[0-9]+[.][0-9]+]
        Int = [[0-9]+]
        String = ["[^\"]*"]
        Identifier = [[a-zA-Z_][a-zA-Z0-9_]*]
    "#;

    let grammar = Grammar::parse(grammar_def);
    let parser = Parser::new(&grammar, code);
    let node = parser.parse(Rule::Program).expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx).expect("Runtime error")
}

#[test]
fn test_string_get() {
    let code = r#"
        s = "abc"
        return get(s, 1)
    "#;
    let result = run_code(code);
    if let Value::String(s) = result {
        assert_eq!(*s.borrow(), "b");
    } else {
        panic!("Expected string, got {:?}", result);
    }
}

#[test]
fn test_string_set() {
    let code = r#"
        s = "abc"
        set(s, 0, "d")
        return s
    "#;
    let result = run_code(code);
    if let Value::String(s) = result {
        assert_eq!(*s.borrow(), "dbc");
    } else {
        panic!("Expected string, got {:?}", result);
    }
}

#[test]
fn test_string_append() {
    let code = r#"
        s = "hi"
        append(s, " there")
        return s
    "#;
    let result = run_code(code);
    if let Value::String(s) = result {
        assert_eq!(*s.borrow(), "hi there");
    } else {
        panic!("Expected string, got {:?}", result);
    }
}
