use multilang::grammar::{Grammar, Rule};
use multilang::node::{Context, Value};
use multilang::parser::Parser;

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
        // print(p) // print not available in test context unless added
    "#;

    let grammar = get_grammar();
    let parser = Parser::new(&grammar, code);
    let node = parser.parse(Rule::Program).expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx).expect("Failed to run");

    let p = ctx.variables.get("p").expect("Variable p not found");
    if let Value::Object(obj_rc) = p {
        let obj = obj_rc.borrow();
        assert_eq!(obj.class_name, "Point");
        assert_eq!(obj.fields.get("x").unwrap(), &Value::Int(10));
        assert_eq!(obj.fields.get("y").unwrap(), &Value::Int(20));
    } else {
        panic!("Expected object, got {:?}", p);
    }
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
    "#;

    let grammar = get_grammar();
    let parser = Parser::new(&grammar, code);
    let node = parser.parse(Rule::Program).expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx).expect("Failed to run");

    let result = ctx
        .variables
        .get("result")
        .expect("Variable result not found");
    assert_eq!(result, &Value::Int(10));
}

#[test]
fn test_member_access() {
    let code = r#"
        class Box {
            content;
        }
        
        b = new Box(123)
        val = b.content
    "#;

    let grammar = get_grammar();
    let parser = Parser::new(&grammar, code);
    let node = parser.parse(Rule::Program).expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx).expect("Failed to run");

    let val = ctx.variables.get("val").expect("Variable val not found");
    assert_eq!(val, &Value::Int(123));
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
    "#;

    let grammar = get_grammar();
    let parser = Parser::new(&grammar, code);
    let node = parser.parse(Rule::Program).expect("Failed to parse");
    let mut ctx = Context::new();
    node.run(&mut ctx).expect("Failed to run");

    let sum = ctx.variables.get("sum").expect("Variable sum not found");
    assert_eq!(sum, &Value::Int(22));
}
