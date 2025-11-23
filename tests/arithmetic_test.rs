use multilang::grammar::{Grammar, Rule};
use multilang::node::{Context, Value};
use multilang::parser::Parser;

fn test_script(grammar_def: &str, input: &str, expected: Value) {
    let grammar = Grammar::parse(grammar_def);
    let parser = Parser::new(&grammar, input);
    let program_node = parser.parse(Rule::Program).expect("Parsing failed");
    let mut ctx = Context::new();
    let result = program_node.run(&mut ctx).expect("Runtime error");
    assert_eq!(result, expected);
}

#[test]
fn test_arithmetic_basic() {
    // Simple right-recursive grammar for testing basic ops
    let grammar = r#"
        Program = Expr
        Expr = Term
        Term = Factor Add Term | Factor Sub Term | Factor
        Factor = Int Mul Factor | Int Div Factor | Int
        
        Add = [\+]
        Sub = [-]
        Mul = [\*]
        Div = [/]
        Int = [[0-9]+]
    "#;

    test_script(grammar, "1 + 2", Value::Int(3));
    test_script(grammar, "5 - 2", Value::Int(3));
    test_script(grammar, "3 * 4", Value::Int(12));
    test_script(grammar, "10 / 2", Value::Int(5));
}

#[test]
fn test_arithmetic_precedence() {
    // Grammar enforcing precedence: Add/Sub < Mul/Div < Int
    let grammar = r#"
        Program = Expr
        Expr = Term
        
        Term = Factor Add Term | Factor Sub Term | Factor
        Factor = Int Mul Factor | Int Div Factor | Int
        
        Add = [\+]
        Sub = [-]
        Mul = [\*]
        Div = [/]
        Int = [[0-9]+]
    "#;

    // 2 + 3 * 4 = 14 (not 20)
    test_script(grammar, "2 + 3 * 4", Value::Int(14));
    // 2 * 3 + 4 = 10
    test_script(grammar, "2 * 3 + 4", Value::Int(10));
}

mod test_utils;

fn get_arithmetic_grammar() -> Grammar {
    let grammar_def = r#"
        Program = Stmt*
        Stmt = FunctionDef
        Stmt = FunctionCall
        Stmt = Return
        FunctionDef = "fn" name:Identifier "(" params:ParamList ")" "{" body:Block "}"
        FunctionDef = "fn" name:Identifier "(" ")" "{" body:Block "}"
        Block = Stmt*
        FunctionCall = name:Identifier "(" args:ArgList ")"
        FunctionCall = name:Identifier "(" ")"
        
        ParamList = Identifier "," params:ParamList
        ParamList = Identifier
        
        ArgList = Expr
        
        Return = "return" Expr
        
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
fn test_float_literal() {
    let grammar = get_arithmetic_grammar();
    let code = r#"
        print(3.14)
    "#;
    test_utils::run_code_and_check(&grammar, code, "3.14");
}

#[test]
fn test_string_literal() {
    let grammar = get_arithmetic_grammar();
    let code = r#"
        print("hello world")
    "#;
    test_utils::run_code_and_check(&grammar, code, "hello world");
}

#[test]
fn test_float_arithmetic() {
    let grammar = get_arithmetic_grammar();
    let code = r#"
        print(1.5 + 2.5)
    "#;
    test_utils::run_code_and_check(&grammar, code, "4");
}

#[test]
fn test_string_concatenation() {
    let grammar = get_arithmetic_grammar();
    let code = r#"
        print("hello" + " " + "world")
    "#;
    test_utils::run_code_and_check(&grammar, code, "hello world");
}

#[test]
fn test_negative_numbers() {
    // We need to update the grammar in run_code to support UnaryOp for negative numbers
    // But run_code uses a hardcoded grammar.
    // So I will define a new grammar here that includes UnaryOp and test it.
    // Actually, the user asked to "add a test... If they are not implement them".
    // So I should try to use the "standard" grammar if possible, but run_code's grammar is local.
    // I will update run_code's grammar to match main.rs more closely, including UnaryOp.

    let grammar_def = r#"
        Program = Stmt*
        Stmt = Print | Return | Expr
        Print = "print" "(" Expr ")"
        Return = "return" Expr
        
        Expr = Term
        Term = Factor Add Term | Factor Sub Term | Factor
        Factor = Unary Mul Factor | Unary Div Factor | Unary
        Unary = UnaryOp Unary | Atom
        Atom = Float | Int | "(" Expr ")"
        
        UnaryOp = [-]
        Add = [\+]
        Sub = [-]
        Mul = [\*]
        Div = [/]
        
        Float = [[0-9]+\.[0-9]+]
        Int = [[0-9]+]
    "#;

    let grammar = Grammar::parse(grammar_def);

    // Test negative int
    let code = "print(-5)";
    test_utils::run_code_and_check(&grammar, code, "-5");

    // Test negative float
    let code = "print(-3.14)";
    test_utils::run_code_and_check(&grammar, code, "-3.14");

    // Test arithmetic with negative
    let code = "print(5 + -3)";
    test_utils::run_code_and_check(&grammar, code, "2");
}

#[test]
fn test_modulo() {
    let grammar_def = r#"
        Program = Stmt*
        Stmt = Print | Return | Expr
        Print = "print" "(" Expr ")"
        Return = "return" Expr
        
        Expr = Term
        Term = Factor Add Term | Factor Sub Term | Factor
        Factor = Unary Mul Factor | Unary Div Factor | Unary Mod Factor | Unary
        Unary = UnaryOp Unary | Atom
        Atom = Float | Int | FunctionCall
        
        FunctionCall = name:Identifier "(" args:ArgList ")"
        ArgList = Expr
        Identifier = [[a-zA-Z_][a-zA-Z0-9_]*]

        UnaryOp = [!] | [-]
        Add = [\+]
        Sub = [-]
        Mul = [\*]
        Div = [/]
        Mod = [%]
        
        Float = [[0-9]+\.[0-9]+]
        Int = [[0-9]+]
    "#;

    let grammar = Grammar::parse(grammar_def);

    let code = "print(10 % 3)";
    test_utils::run_code_and_check(&grammar, code, "1");
}

#[test]
fn test_configurable_arithmetic() {
    let grammar_def = r#"
        Program = Stmt*
        Stmt = Print | Return | WhileLoop | IfElse | IfThen | FunctionDef | FunctionCall | Assignment | Expr
        
        Print = "print" "(" Expr ")"
        Return = "return" value:Expr

        WhileLoop = "while" condition:Expr "{" body:Block "}"

        IfElse = "if" condition:Expr "{" then:Block "}" "else" "{" else:Block "}"
        IfThen = "if" condition:Expr "{" then:Block "}"


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
        Factor = Unary Mul Factor | Unary Div Factor | Unary Mod Factor | Unary
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
        Add = [\+] | [plus]
        Sub = [-] | [minus]
        Mul = [\*] | [times]
        Div = [/] | [divide]
        Mod = [%] | [modulo]

        Float = [[0-9]+\.[0-9]+]
        Int = [[0-9]+]
        String = ["[^"]*"]
        Identifier = [[a-zA-Z_][a-zA-Z0-9_]*]
    "#;
    let grammar = Grammar::parse(&grammar_def);

    // Test "plus"
    let code = "print(4 plus 5)";
    test_utils::run_code_and_check(&grammar, code, "9");

    // Test "minus"
    let code = "print(10 minus 2)";
    test_utils::run_code_and_check(&grammar, code, "8");

    // Test "times"
    let code = "print(3 times 3)";
    test_utils::run_code_and_check(&grammar, code, "9");

    // Test "divide"
    let code = "print(20 divide 4)";
    test_utils::run_code_and_check(&grammar, code, "5");

    // Test "modulo"
    let code = "print(10 modulo 3)";
    test_utils::run_code_and_check(&grammar, code, "1");
}
