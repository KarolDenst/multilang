use multilang::grammar::Grammar;
use multilang::node::{Context, Value};
use multilang::parser::Parser;

fn run_code(code: &str) -> Value {
    let grammar_def = r#"
        Program = Stmt*
        Stmt = FunctionCall
        Stmt = Return
        FunctionCall = name:Identifier "(" args:ArgList ")"
        FunctionCall = name:Identifier "(" ")"
        ArgList = Expr "," args:ArgList
        ArgList = Expr
        Return = "return" Expr
        
        Expr = LogicalOr
        LogicalOr = LogicalAnd "||" LogicalOr | LogicalAnd
        LogicalAnd = Comparison "&&" LogicalAnd | Comparison
        Comparison = Term Eq Term | Term Neq Term | Term Lt Term | Term Gt Term | Term
        Term = Factor Add Term | Factor Sub Term | Factor
        Factor = Unary Mul Factor | Unary Div Factor | Unary
        Unary = UnaryOp Unary | Atom
        Atom = Float | Int | String | Identifier | FunctionCall | "(" Expr ")"
        
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

    let grammar = Grammar::parse(grammar_def);
    let parser = Parser::new(&grammar, code);
    let node = parser.parse("Program").expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx)
}

#[test]
fn test_and() {
    assert_eq!(run_code("return 1 == 1 && 2 == 2"), Value::Bool(true));
    assert_eq!(run_code("return 1 == 1 && 1 == 2"), Value::Bool(false));
    assert_eq!(run_code("return 1 == 2 && 1 == 1"), Value::Bool(false));
    assert_eq!(run_code("return 1 == 2 && 1 == 2"), Value::Bool(false));
}

#[test]
fn test_or() {
    assert_eq!(run_code("return 1 == 1 || 2 == 2"), Value::Bool(true));
    assert_eq!(run_code("return 1 == 1 || 1 == 2"), Value::Bool(true));
    assert_eq!(run_code("return 1 == 2 || 1 == 1"), Value::Bool(true));
    assert_eq!(run_code("return 1 == 2 || 1 == 2"), Value::Bool(false));
}

#[test]
fn test_not() {
    assert_eq!(run_code("return ! (1 == 2)"), Value::Bool(true));
    assert_eq!(run_code("return ! (1 == 1)"), Value::Bool(false));
}

#[test]
fn test_precedence() {
    // && has higher precedence than ||
    // true || false && false -> true || (false && false) -> true || false -> true
    assert_eq!(run_code("return 1==1 || 1==2 && 1==2"), Value::Bool(true));

    // ! has higher precedence than &&
    // !true && false -> false && false -> false
    assert_eq!(run_code("return ! (1==1) && (1==2)"), Value::Bool(false));

    // !false && true -> true && true -> true
    assert_eq!(run_code("return ! (1==2) && (1==1)"), Value::Bool(true));
}
