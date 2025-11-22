use multilang::grammar::{Grammar, Rule};
use multilang::node::{Context, Value};
use multilang::parser::Parser;
use std::cell::RefCell;
use std::rc::Rc;

fn get_grammar() -> Grammar {
    let grammar_def = r##"
        Program = Stmt*
        Stmt = FunctionDef | FunctionCall | Assignment | Expr
        
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

fn run_code(code: &str) -> Context {
    let grammar = get_grammar();
    let parser = Parser::new(&grammar, code);
    let node = parser.parse(Rule::Program).expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx).expect("Failed to run");
    ctx
}

#[test]
fn test_len_string() {
    let code = r#"result = len("hello")"#;
    let ctx = run_code(code);
    assert_eq!(ctx.variables.get("result"), Some(&Value::Int(5)));
}

#[test]
fn test_len_list() {
    let code = r#"
        lst = [1, 2, 3, 4, 5]
        result = len(lst)
    "#;
    let ctx = run_code(code);
    assert_eq!(ctx.variables.get("result"), Some(&Value::Int(5)));
}

#[test]
fn test_abs_int() {
    let code = r#"
        neg_five = 0 - 5
        result1 = abs(neg_five)
        result2 = abs(5)
    "#;
    let ctx = run_code(code);
    assert_eq!(ctx.variables.get("result1"), Some(&Value::Int(5)));
    assert_eq!(ctx.variables.get("result2"), Some(&Value::Int(5)));
}

#[test]
fn test_abs_float() {
    let code = r#"
        neg_pi = 0.0 - 3.14
        result1 = abs(neg_pi)
        result2 = abs(3.14)
    "#;
    let ctx = run_code(code);
    assert_eq!(ctx.variables.get("result1"), Some(&Value::Float(3.14)));
    assert_eq!(ctx.variables.get("result2"), Some(&Value::Float(3.14)));
}

#[test]
fn test_sum() {
    let code = r#"
        nums = [1, 2, 3, 4, 5]
        result = sum(nums)
    "#;
    let ctx = run_code(code);
    assert_eq!(ctx.variables.get("result"), Some(&Value::Int(15)));
}

#[test]
fn test_sum_with_floats() {
    let code = r#"
        nums = [1, 2.5, 3, 4.5]
        result = sum(nums)
    "#;
    let ctx = run_code(code);
    assert_eq!(ctx.variables.get("result"), Some(&Value::Float(11.0)));
}

#[test]
fn test_range_one_arg() {
    let code = r#"
        result = range(5)
    "#;
    let ctx = run_code(code);
    let expected = Value::List(Rc::new(RefCell::new(vec![
        Value::Int(0),
        Value::Int(1),
        Value::Int(2),
        Value::Int(3),
        Value::Int(4),
    ])));
    assert_eq!(ctx.variables.get("result"), Some(&expected));
}

#[test]
fn test_range_two_args() {
    let code = r#"
        result = range(2, 7)
    "#;
    let ctx = run_code(code);
    let expected = Value::List(Rc::new(RefCell::new(vec![
        Value::Int(2),
        Value::Int(3),
        Value::Int(4),
        Value::Int(5),
        Value::Int(6),
    ])));
    assert_eq!(ctx.variables.get("result"), Some(&expected));
}

#[test]
fn test_slice_string() {
    let code = r#"
        s = "hello world"
        result = slice(s, 0, 5)
    "#;
    let ctx = run_code(code);
    assert_eq!(
        ctx.variables.get("result"),
        Some(&Value::String(Rc::new(RefCell::new("hello".to_string()))))
    );
}

#[test]
fn test_slice_list() {
    let code = r#"
        lst = [1, 2, 3, 4, 5]
        result = slice(lst, 1, 4)
    "#;
    let ctx = run_code(code);
    let expected = Value::List(Rc::new(RefCell::new(vec![
        Value::Int(2),
        Value::Int(3),
        Value::Int(4),
    ])));
    assert_eq!(ctx.variables.get("result"), Some(&expected));
}

#[test]
fn test_split() {
    let code = r#"
        s = "hello,world,test"
        result = split(s, ",")
    "#;
    let ctx = run_code(code);
    let expected = Value::List(Rc::new(RefCell::new(vec![
        Value::String(Rc::new(RefCell::new("hello".to_string()))),
        Value::String(Rc::new(RefCell::new("world".to_string()))),
        Value::String(Rc::new(RefCell::new("test".to_string()))),
    ])));
    assert_eq!(ctx.variables.get("result"), Some(&expected));
}

#[test]
fn test_join() {
    let code = r#"
        lst = ["hello", "world"]
        result = join(lst, " ")
    "#;
    let ctx = run_code(code);
    assert_eq!(
        ctx.variables.get("result"),
        Some(&Value::String(Rc::new(RefCell::new(
            "hello world".to_string()
        ))))
    );
}

#[test]
fn test_join_with_numbers() {
    let code = r#"
        lst = [1, 2, 3]
        result = join(lst, "-")
    "#;
    let ctx = run_code(code);
    assert_eq!(
        ctx.variables.get("result"),
        Some(&Value::String(Rc::new(RefCell::new("1-2-3".to_string()))))
    );
}

#[test]
fn test_reverse() {
    let code = r#"
        lst = [1, 2, 3, 4, 5]
        reverse(lst)
    "#;
    let ctx = run_code(code);
    let lst = ctx.variables.get("lst").unwrap();
    if let Value::List(l) = lst {
        let list = l.borrow();
        assert_eq!(list.len(), 5);
        assert_eq!(list[0], Value::Int(5));
        assert_eq!(list[1], Value::Int(4));
        assert_eq!(list[2], Value::Int(3));
        assert_eq!(list[3], Value::Int(2));
        assert_eq!(list[4], Value::Int(1));
    } else {
        panic!("Expected list");
    }
}

#[test]
fn test_sort_ints() {
    let code = r#"
        lst = [5, 2, 8, 1, 9]
        sort(lst)
    "#;
    let ctx = run_code(code);
    let lst = ctx.variables.get("lst").unwrap();
    if let Value::List(l) = lst {
        let list = l.borrow();
        assert_eq!(list.len(), 5);
        assert_eq!(list[0], Value::Int(1));
        assert_eq!(list[1], Value::Int(2));
        assert_eq!(list[2], Value::Int(5));
        assert_eq!(list[3], Value::Int(8));
        assert_eq!(list[4], Value::Int(9));
    } else {
        panic!("Expected list");
    }
}

#[test]
fn test_sort_strings() {
    let code = r#"
        lst = ["zebra", "apple", "mango"]
        sort(lst)
    "#;
    let ctx = run_code(code);
    let lst = ctx.variables.get("lst").unwrap();
    if let Value::List(l) = lst {
        let list = l.borrow();
        assert_eq!(list.len(), 3);
        if let Value::String(s) = &list[0] {
            assert_eq!(*s.borrow(), "apple");
        }
        if let Value::String(s) = &list[1] {
            assert_eq!(*s.borrow(), "mango");
        }
        if let Value::String(s) = &list[2] {
            assert_eq!(*s.borrow(), "zebra");
        }
    } else {
        panic!("Expected list");
    }
}

#[test]
fn test_ord() {
    let code = r#"
        result1 = ord("A")
        result2 = ord("a")
    "#;
    let ctx = run_code(code);
    assert_eq!(ctx.variables.get("result1"), Some(&Value::Int(65)));
    assert_eq!(ctx.variables.get("result2"), Some(&Value::Int(97)));
}

#[test]
fn test_chr() {
    let code = r#"
        result1 = chr(65)
        result2 = chr(97)
    "#;
    let ctx = run_code(code);
    assert_eq!(
        ctx.variables.get("result1"),
        Some(&Value::String(Rc::new(RefCell::new("A".to_string()))))
    );
    assert_eq!(
        ctx.variables.get("result2"),
        Some(&Value::String(Rc::new(RefCell::new("a".to_string()))))
    );
}

#[test]
fn test_ord_chr_roundtrip() {
    let code = r#"
        original = "X"
        code_point = ord(original)
        restored = chr(code_point)
    "#;
    let ctx = run_code(code);
    assert_eq!(ctx.variables.get("code_point"), Some(&Value::Int(88)));
    assert_eq!(
        ctx.variables.get("restored"),
        Some(&Value::String(Rc::new(RefCell::new("X".to_string()))))
    );
}
