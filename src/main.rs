use multilang::grammar::Grammar;
use multilang::parser::Parser;
use multilang::node::Context;

fn main() {
    // 1. Define Grammar
    let grammar_def = r#"
        Program = Stmt*
        Stmt = FunctionDef
        Stmt = FunctionCall
        Stmt = Return
        FunctionDef = "fn" name:Identifier "(" params:ParamList ")" "{" body:Block "}"
        Block = Stmt*
        FunctionCall = name:Identifier "(" args:ArgList ")"
        ParamList = Identifier | Identifier "," ParamList
        ArgList = Expr | Expr "," ArgList
        Return = "return" Int
        Expr = Int | Identifier | FunctionCall
        Int = [[0-9]+]
        Identifier = [[a-zA-Z_][a-zA-Z0-9_]*]
    "#;
    
    // Note: The recursive definition of ParamList/ArgList above might be tricky for my simple parser.
    // My parser handles `Identifier ("," Identifier)*` if I use `*` or similar?
    // My parser supports `*` for repetition.
    // But `Identifier ("," Identifier)*` is not directly supported by my simple grammar engine which only supports `*` on single rule reference?
    // Let's check `grammar.rs`.
    // `Star(Box<Pattern>)`
    // `Pattern` can be `RuleReference`.
    // So `Rule*` is supported.
    // But `("," Identifier)*` involves a sequence inside `*`.
    // My grammar engine might not support grouping `()`.
    // Let's check `grammar.rs` again.
    // It parses `[...]` as Regex, `"*"` as Star of previous token.
    // It doesn't seem to support `(...)`.
    
    // So I cannot define `ParamList = Identifier ("," Identifier)*`.
    // I have to define it recursively or using a helper rule if I can.
    // `ParamList = Identifier`
    // `ParamList = Identifier "," ParamList`
    // This is right-recursive.
    // My parser is top-down recursive descent?
    // `parse_rule` iterates alternatives.
    // If I have:
    // ParamList = Identifier "," ParamList
    // ParamList = Identifier
    // It will try the first one. If it matches Identifier and ",", it recurses.
    // This should work for `a, b, c`.
    // `a` matches Identifier. `,` matches. Recurse for `b, c`.
    // `b` matches Identifier. `,` matches. Recurse for `c`.
    // `c` matches Identifier. `,` fails. Backtrack?
    // Wait, if `Identifier "," ParamList` fails (because no comma), it should try `Identifier`.
    // So I should put the recursive case first?
    // Yes.
    
    // But wait, `ArgList` can be empty?
    // `FunctionCall = name:Identifier "(" args:ArgList ")"`
    // If `ArgList` is mandatory, then `()` will fail if no args.
    // I need `FunctionCall = name:Identifier "(" ")"` for empty args.
    // And `FunctionCall = name:Identifier "(" args:ArgList ")"` for args.
    
    // Let's refine the grammar in `main.rs`.

    let grammar_def = r#"
        Program = Stmt*
        Stmt = FunctionDef
        Stmt = FunctionCall
        Stmt = Print
        Stmt = Return
        FunctionDef = "fn" name:Identifier "(" params:ParamList ")" "{" body:Block "}"
        FunctionDef = "fn" name:Identifier "(" ")" "{" body:Block "}"
        Block = Stmt*
        FunctionCall = name:Identifier "(" args:ArgList ")"
        FunctionCall = name:Identifier "(" ")"
        
        ParamList = Identifier "," params:ParamList
        ParamList = Identifier
        
        ArgList = Expr "," args:ArgList
        ArgList = Expr
        
        Return = "return" Expr
        
        Expr = LogicalOr
        LogicalOr = LogicalAnd "||" LogicalOr | LogicalAnd
        LogicalAnd = Comparison "&&" LogicalAnd | Comparison
        Comparison = Term CompOp Term | Term
        Term = Factor AddOp Term | Factor
        Factor = Unary MulOp Factor | Unary
        Unary = UnaryOp Unary | Atom
        Atom = Float | Int | String | Identifier | FunctionCall | "(" Expr ")"
        
        UnaryOp = [!]
        CompOp = [==] | [!=] | [<] | [>]
        AddOp = [\+] | [-]
        MulOp = [\*] | [/]
        
        Float = [[0-9]+\.[0-9]+]
        Int = [[0-9]+]
        String = ["[^\"]*"]
        Identifier = [[a-zA-Z_][a-zA-Z0-9_]*]
    "#;

    println!("Parsing grammar...");
    let grammar = Grammar::parse(grammar_def);

    // 2. Define Input Script
    let input = r#"
        fn add(a, b) {
            return a
        }
        print(100)
        add(10, 20)
    "#;
    // Note: `return a` returns `a`. `a` is Identifier.
    // `Identifier` node needs to look up variable in Context.
    // I haven't implemented `Identifier` node run method to look up variable!
    // `Parser` creates `RawTokenNode` for Identifier?
    // No, `Parser` has `Identifier` rule which returns `RawTokenNode` (or just child).
    // `RawTokenNode` returns `Void`.
    // I need a `Variable` node or update `Identifier` handling.
    
    // Let's check `parser.rs` for `Identifier`.
    // `Identifier` => `child`. `child` is `RawTokenNode`.
    // `RawTokenNode::run` returns `Void`.
    // I need to change this.
    
    // I will update `main.rs` but I expect it to fail or return Void for `a`.
    // I need to fix `Identifier` execution first.
    
    // But let's write the `main.rs` first to confirm the grammar works.

    // 3. Parse Input
    println!("Parsing input: '{}'", input);
    let parser = Parser::new(&grammar, input);
    match parser.parse("Program") {
        Ok(program_node) => {
            println!("Parsing successful! Running program...");
            // 4. Run Program
            let mut ctx = Context::new();
            let result = program_node.run(&mut ctx);
            println!("Program returned: {:?}", result);
            
            // Expect 10
             if let multilang::node::Value::Int(val) = result {
                assert_eq!(val, 10);
                println!("Assertion passed: Returned 10");
            } else {
                println!("Assertion failed: Expected Int(10), got {:?}", result);
            }
        }
        Err(e) => {
            println!("Parsing failed: {}", e);
        }
    }
}
