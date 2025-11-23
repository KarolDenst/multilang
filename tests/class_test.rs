use multilang::grammar::{Grammar, Rule};
use multilang::node::Context;
use multilang::parser::Parser;

mod test_utils;

fn get_grammar() -> Grammar {
    let grammar_def = r##"
        Program = Stmt*
        Stmt = ClassDef | FunctionDef | FunctionCall | Return | Assignment | Expr
        
        ClassDef = "class" name:Identifier "{" ClassMember* "}"
        ClassMember = FieldDef | MethodDef
        FieldDef = name:Identifier ";"
        MethodDef = "fn" name:Identifier "(" params:ParamList ")" "{" body:Block "}"
        MethodDef = "fn" name:Identifier "(" ")" "{" body:Block "}"
        
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
        Unary = UnaryOp Unary | Postfix
        
        Postfix = Atom PostfixSuffix*
        PostfixSuffix = "." method:Identifier "(" args:ArgList ")"
        PostfixSuffix = "." method:Identifier "(" ")"
        PostfixSuffix = "." member:Identifier
        
        Atom = NewExpr | SelfReference | Float | Int | String | FunctionCall | Identifier | ListLiteral | "(" Expr ")"
        
        NewExpr = "new" class_name:Identifier "(" args:ArgList ")"
        NewExpr = "new" class_name:Identifier "(" ")"
        
        SelfReference = "this"
        
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
        Mod = [%]
        
        Float = [[0-9]+\.[0-9]+]
        Int = [[0-9]+]
        String = ["[^\"]*"]
        Identifier = [[a-zA-Z_][a-zA-Z0-9_]*]
    "##;
    Grammar::parse(grammar_def)
}

#[test]
fn test_class_definition_and_instantiation() {
    let code = r#"
        class Point {
            x;
            y;
        }
        
        p = new Point(10, 20)
        print(p)
    "#;

    let (logs, _guard) = test_utils::capture_output();
    let grammar = get_grammar();
    let parser = Parser::new(&grammar, code);
    let node = parser.parse(Rule::Program).expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx).expect("Failed to run");

    assert_eq!(*logs.borrow(), vec!["<Object Point>\n"]);
}

#[test]
fn test_method_call() {
    let code = r#"
        class Calculator {
            factor;
            
            fn multiply(x) {
                return x * this.factor
            }
        }
        
        calc = new Calculator(2)
        result = calc.multiply(5)
        print(result)
    "#;

    let (logs, _guard) = test_utils::capture_output();
    let grammar = get_grammar();
    let parser = Parser::new(&grammar, code);
    let node = parser.parse(Rule::Program).expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx).expect("Failed to run");

    assert_eq!(*logs.borrow(), vec!["10\n"]);
}

#[test]
fn test_member_access() {
    let code = r#"
        class Box {
            content;
        }
        
        b = new Box(123)
        val = b.content
        print(val)
    "#;

    let (logs, _guard) = test_utils::capture_output();
    let grammar = get_grammar();
    let parser = Parser::new(&grammar, code);
    let node = parser.parse(Rule::Program).expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx).expect("Failed to run");

    assert_eq!(*logs.borrow(), vec!["123\n"]);
}

#[test]
fn test_method_call_with_multiple_args() {
    let code = r#"
        class Adder {
            base;
            
            fn add(a, b) {
                return this.base + a + b
            }
        }
        
        adder = new Adder(10)
        sum = adder.add(5, 7)
        print(sum)
    "#;

    let (logs, _guard) = test_utils::capture_output();
    let grammar = get_grammar();
    let parser = Parser::new(&grammar, code);
    let node = parser.parse(Rule::Program).expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx).expect("Failed to run");

    assert_eq!(*logs.borrow(), vec!["22\n"]);
}
