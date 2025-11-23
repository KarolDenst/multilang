use multilang::grammar::Grammar;

mod test_utils;

fn get_comparison_grammar() -> Grammar {
    let grammar_def = r#"
        Program = Stmt*
        Stmt = Print
        Stmt = FunctionCall
        Stmt = Return
        Print = "print" "(" Expr ")"
        FunctionCall = name:Identifier "(" args:ArgList ")"
        FunctionCall = name:Identifier "(" ")"
        ArgList = Expr "," args:ArgList
        ArgList = Expr
        Return = "return" Expr
        
        Expr = Comparison
        Comparison = Term Eq Term | Term Neq Term | Term Lt Term | Term Gt Term | Term
        Term = Factor Add Term | Factor Sub Term | Factor
        Factor = Atom Mul Factor | Atom Div Factor | Atom
        Atom = Float | Int | String | Identifier | FunctionCall | "(" Expr ")"
        
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
    Grammar::parse(grammar_def)
}

#[test]
fn test_int_equality() {
    let grammar = get_comparison_grammar();
    test_utils::run_code_and_check(&grammar, "print(1 == 1)", "true");
    test_utils::run_code_and_check(&grammar, "print(1 == 2)", "false");
    test_utils::run_code_and_check(&grammar, "print(1 != 2)", "true");
    test_utils::run_code_and_check(&grammar, "print(1 != 1)", "false");
}

#[test]
fn test_int_comparison() {
    let grammar = get_comparison_grammar();
    test_utils::run_code_and_check(&grammar, "print(1 < 2)", "true");
    test_utils::run_code_and_check(&grammar, "print(2 < 1)", "false");
    test_utils::run_code_and_check(&grammar, "print(2 > 1)", "true");
    test_utils::run_code_and_check(&grammar, "print(1 > 2)", "false");
}

#[test]
fn test_float_comparison() {
    let grammar = get_comparison_grammar();
    test_utils::run_code_and_check(&grammar, "print(1.0 == 1.0)", "true");
    test_utils::run_code_and_check(&grammar, "print(1.0 != 2.0)", "true");
    test_utils::run_code_and_check(&grammar, "print(1.0 < 2.0)", "true");
    test_utils::run_code_and_check(&grammar, "print(2.0 > 1.0)", "true");
}

#[test]
fn test_string_comparison() {
    let grammar = get_comparison_grammar();
    test_utils::run_code_and_check(&grammar, "print(\"a\" == \"a\")", "true");
    test_utils::run_code_and_check(&grammar, "print(\"a\" != \"b\")", "true");
    test_utils::run_code_and_check(&grammar, "print(\"a\" < \"b\")", "true");
    test_utils::run_code_and_check(&grammar, "print(\"b\" > \"a\")", "true");
}

#[test]
fn test_precedence() {
    let grammar = get_comparison_grammar();
    // 1 + 2 < 4 -> 3 < 4 -> True
    test_utils::run_code_and_check(&grammar, "print(1 + 2 < 4)", "true");
    // 1 + 2 == 3 -> 3 == 3 -> True
    test_utils::run_code_and_check(&grammar, "print(1 + 2 == 3)", "true");
}

#[test]
fn test_configurable_comparison() {
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

    // Test "4 plus 5 == 9"
    let code = "print(4 plus 5 == 9)";
    test_utils::run_code_and_check(&grammar, code, "true");
}
