# Example

This document provides a complete example of a grammar definition and corresponding code.

## Grammar

```rust
Program = Stmt*
Stmt = FunctionDef | FunctionCall | Return | If | Print
FunctionDef = "fn" name:Identifier "(" params:ParamList ")" "{" body:Block "}"
FunctionDef = "fn" name:Identifier "(" ")" "{" body:Block "}"
Block = "{" Program "}"
FunctionCall = name:Identifier "(" args:ArgList ")"
FunctionCall = name:Identifier "(" ")"
ParamList = Identifier "," params:ParamList
ParamList = Identifier
ArgList = Expr "," args:ArgList
ArgList = Expr
Return = "return" Expr
If = "if" condition:Expr then:Block "else" else:Block
If = "if" condition:Expr then:Block
Print = "print" "(" Expr ")"

Expr = LogicalOr
LogicalOr = LogicalAnd "||" LogicalOr | LogicalAnd
LogicalAnd = Comparison "&&" LogicalAnd | Comparison
Comparison = Term Eq Term | Term Neq Term | Term Lt Term | Term Gt Term | Term
Term = Factor Add Term | Factor Sub Term | Factor
Factor = Unary Mul Factor | Unary Div Factor | Unary
Unary = UnaryOp Unary | Atom
Atom = Float | Int | String | Identifier | FunctionCall | "(" Expr ")" | True | False

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
True = "true"
False = "false"
```

## Code

```javascript
fn fib(n) {
    if n < 2 {
        return n
    } else {
        return fib(n - 1) + fib(n - 2)
    }
}

print(fib(10))
```
