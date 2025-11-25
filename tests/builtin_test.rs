use multilang::grammar::Grammar;

use crate::test_utils::run_code_and_check;

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

mod test_utils;

#[test]
fn test_len_string() {
    let grammar = get_grammar();
    let code = r#"
        result = len("hello")
        print(result)
    "#;
    run_code_and_check(&grammar, code, "5");
}

#[test]
fn test_len_list() {
    let grammar = get_grammar();
    let code = r#"
        lst = [1, 2, 3, 4, 5]
        result = len(lst)
        print(result)
    "#;
    run_code_and_check(&grammar, code, "5");
}

#[test]
fn test_abs_int() {
    let grammar = get_grammar();
    let code = r#"
        neg_five = 0 - 5
        result1 = abs(neg_five)
        result2 = abs(5)
        print(result1)
        print(result2)
    "#;
    run_code_and_check(&grammar, code, "5\n5");
}

#[test]
fn test_abs_float() {
    let grammar = get_grammar();
    let code = r#"
        neg_pi = 0.0 - 3.14
        result1 = abs(neg_pi)
        result2 = abs(3.14)
        print(result1)
        print(result2)
    "#;
    run_code_and_check(&grammar, code, "3.14\n3.14");
}

#[test]
fn test_sum() {
    let grammar = get_grammar();
    let code = r#"
        nums = [1, 2, 3, 4, 5]
        result = sum(nums)
        print(result)
    "#;
    run_code_and_check(&grammar, code, "15");
}

#[test]
fn test_sum_with_floats() {
    let grammar = get_grammar();
    let code = r#"
        nums = [1, 2.5, 3, 4.5]
        result = sum(nums)
        print(result)
    "#;
    run_code_and_check(&grammar, code, "11");
}

#[test]
fn test_range_one_arg() {
    let grammar = get_grammar();
    let code = r#"
        result = range(5)
        print(result)
    "#;
    run_code_and_check(&grammar, code, "[0, 1, 2, 3, 4]");
}

#[test]
fn test_range_two_args() {
    let grammar = get_grammar();
    let code = r#"
        result = range(2, 7)
        print(result)
    "#;
    run_code_and_check(&grammar, code, "[2, 3, 4, 5, 6]");
}

#[test]
fn test_slice_string() {
    let grammar = get_grammar();
    let code = r#"
        s = "hello world"
        result = slice(s, 0, 5)
        print(result)
    "#;
    run_code_and_check(&grammar, code, "hello");
}

#[test]
fn test_slice_list() {
    let grammar = get_grammar();
    let code = r#"
        lst = [1, 2, 3, 4, 5]
        result = slice(lst, 1, 4)
        print(result)
    "#;
    run_code_and_check(&grammar, code, "[2, 3, 4]");
}

#[test]
fn test_split() {
    let grammar = get_grammar();
    let code = r#"
        s = "hello,world,test"
        result = split(s, ",")
        print(result)
    "#;
    run_code_and_check(&grammar, code, "[hello, world, test]");
}

#[test]
fn test_join() {
    let grammar = get_grammar();
    let code = r#"
        lst = ["hello", "world"]
        result = join(lst, " ")
        print(result)
    "#;
    run_code_and_check(&grammar, code, "hello world");
}

#[test]
fn test_join_with_numbers() {
    let grammar = get_grammar();
    let code = r#"
        lst = [1, 2, 3]
        result = join(lst, "-")
        print(result)
    "#;
    run_code_and_check(&grammar, code, "1-2-3");
}

#[test]
fn test_reverse() {
    let grammar = get_grammar();
    let code = r#"
        lst = [1, 2, 3, 4, 5]
        reverse(lst)
        print(lst)
    "#;
    run_code_and_check(&grammar, code, "[5, 4, 3, 2, 1]");
}

#[test]
fn test_sort_ints() {
    let grammar = get_grammar();
    let code = r#"
        lst = [5, 2, 8, 1, 9]
        sort(lst)
        print(lst)
    "#;
    run_code_and_check(&grammar, code, "[1, 2, 5, 8, 9]");
}

#[test]
fn test_sort_strings() {
    let grammar = get_grammar();
    let code = r#"
        lst = ["zebra", "apple", "mango"]
        sort(lst)
        print(lst)
    "#;
    run_code_and_check(&grammar, code, "[apple, mango, zebra]");
}

#[test]
fn test_ord() {
    let grammar = get_grammar();
    let code = r#"
        result1 = ord("A")
        result2 = ord("a")
        print(result1)
        print(result2)
    "#;
    run_code_and_check(&grammar, code, "65\n97");
}

#[test]
fn test_chr() {
    let grammar = get_grammar();
    let code = r#"
        result1 = chr(65)
        result2 = chr(97)
        print(result1)
        print(result2)
    "#;
    run_code_and_check(&grammar, code, "A\na");
}

#[test]
fn test_ord_chr_roundtrip() {
    let grammar = get_grammar();
    let code = r#"
        original = "X"
        code_point = ord(original)
        restored = chr(code_point)
        print(code_point)
        print(restored)
    "#;
    run_code_and_check(&grammar, code, "88\nX");
}
