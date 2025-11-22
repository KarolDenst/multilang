use multilang::grammar::Grammar;
use multilang::node::{Context, Value};
use multilang::parser::Parser;

fn get_grammar() -> Grammar {
    let grammar_def = r##"
        Program = Stmt*
        Stmt = FunctionDef | FunctionCall | Print | Return | Assignment
        FunctionDef = "fn" name:Identifier "(" params:ParamList ")" "{" body:Block "}"
        FunctionDef = "fn" name:Identifier "(" ")" "{" body:Block "}"
        Block = Stmt*
        FunctionCall = name:Identifier "(" args:ArgList ")"
        FunctionCall = name:Identifier "(" ")"
        
        Assignment = name:Identifier "=" value:Expr
        
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
fn test_list_creation() {
    let code = "x = [1, 2, 3]";
    let grammar = get_grammar();
    let parser = Parser::new(&grammar, code);
    let node = parser.parse("Program").expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx).expect("Failed to run");

    let x = ctx.variables.get("x").expect("Variable x not found");
    if let Value::List(list) = x {
        let list = list.borrow();
        assert_eq!(list.len(), 3);
        assert_eq!(list[0], Value::Int(1));
        assert_eq!(list[1], Value::Int(2));
        assert_eq!(list[2], Value::Int(3));
    } else {
        panic!("x is not a list");
    }
}

#[test]
fn test_list_append() {
    let code = "x = [1] append(x, 2)";
    let grammar = get_grammar();
    let parser = Parser::new(&grammar, code);
    let node = parser.parse("Program").expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx).expect("Failed to run");

    let x = ctx.variables.get("x").expect("Variable x not found");
    if let Value::List(list) = x {
        let list = list.borrow();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0], Value::Int(1));
        assert_eq!(list[1], Value::Int(2));
    } else {
        panic!("x is not a list");
    }
}

#[test]
fn test_list_get() {
    let code = "x = [10, 20] y = get(x, 1)";
    let grammar = get_grammar();
    let parser = Parser::new(&grammar, code);
    let node = parser.parse("Program").expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx).expect("Failed to run");

    let y = ctx.variables.get("y").expect("Variable y not found");
    assert_eq!(*y, Value::Int(20));
}

#[test]
fn test_nested_list() {
    let code = "x = [[1], [2]]";
    let grammar = get_grammar();
    let parser = Parser::new(&grammar, code);
    let node = parser.parse("Program").expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx).expect("Failed to run");

    let x = ctx.variables.get("x").expect("Variable x not found");
    if let Value::List(list) = x {
        let list = list.borrow();
        assert_eq!(list.len(), 2);
        println!("List[0]: {:?}", list[0]);
        if let Value::List(sublist1) = &list[0] {
            assert_eq!(sublist1.borrow()[0], Value::Int(1));
        } else {
            panic!("First element is not a list: {:?}", list[0]);
        }
    } else {
        panic!("x is not a list");
    }
}

#[test]
fn test_list_mutability() {
    let code = "x = [1] y = x append(y, 2)";
    let grammar = get_grammar();
    let parser = Parser::new(&grammar, code);
    let node = parser.parse("Program").expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx).expect("Failed to run");

    let x = ctx.variables.get("x").expect("Variable x not found");
    if let Value::List(list) = x {
        let list = list.borrow();
        assert_eq!(list.len(), 2);
        assert_eq!(list[1], Value::Int(2));
    } else {
        panic!("x is not a list");
    }
}

#[test]
fn test_list_set() {
    let code = "l = [1, 2] set(l, 0, 10) x = get(l, 0)";
    let grammar = get_grammar();
    let parser = Parser::new(&grammar, code);
    let node = parser.parse("Program").expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx).expect("Failed to run");

    let x = ctx.variables.get("x").expect("Variable x not found");
    assert_eq!(*x, Value::Int(10));
}
