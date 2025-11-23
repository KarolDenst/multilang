use multilang::grammar::Grammar;

mod test_utils;

fn get_logical_grammar() -> Grammar {
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
        String = ["[^"]*"]
        Identifier = [[a-zA-Z_][a-zA-Z0-9_]*]
    "#;
    Grammar::parse(grammar_def)
}

#[test]
fn test_and() {
    let grammar = get_logical_grammar();
    test_utils::run_code_and_check(&grammar, "print(1 == 1 && 2 == 2)", "true");
    test_utils::run_code_and_check(&grammar, "print(1 == 1 && 1 == 2)", "false");
    test_utils::run_code_and_check(&grammar, "print(1 == 2 && 1 == 1)", "false");
    test_utils::run_code_and_check(&grammar, "print(1 == 2 && 1 == 2)", "false");
}

#[test]
fn test_or() {
    let grammar = get_logical_grammar();
    test_utils::run_code_and_check(&grammar, "print(1 == 1 || 2 == 2)", "true");
    test_utils::run_code_and_check(&grammar, "print(1 == 1 || 1 == 2)", "true");
    test_utils::run_code_and_check(&grammar, "print(1 == 2 || 1 == 1)", "true");
    test_utils::run_code_and_check(&grammar, "print(1 == 2 || 1 == 2)", "false");
}

#[test]
fn test_not() {
    let grammar = get_logical_grammar();
    test_utils::run_code_and_check(&grammar, "print(! (1 == 2))", "true");
    test_utils::run_code_and_check(&grammar, "print(! (1 == 1))", "false");
}

#[test]
fn test_precedence() {
    let grammar = get_logical_grammar();
    // && has higher precedence than ||
    // true || false && false -> true || (false && false) -> true || false -> true
    test_utils::run_code_and_check(&grammar, "print(1==1 || 1==2 && 1==2)", "true");

    // ! has higher precedence than &&
    // !true && false -> false && false -> false
    test_utils::run_code_and_check(&grammar, "print(! (1==1) && (1==2))", "false");

    // !false && true -> true && true -> true
    test_utils::run_code_and_check(&grammar, "print(! (1==2) && (1==1))", "true");
}
