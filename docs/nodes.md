# AST Nodes

This document lists all the Abstract Syntax Tree (AST) nodes available in the language and how to define them in the grammar.

## Program Structure
- **Program**: The root node of the AST. Defined as a sequence of statements.
  - Grammar: `Program = Stmt*`
- **Block**: A sequence of statements enclosed in braces.
  - Grammar: `Block = "{" Program "}"`

## Statements
- **FunctionDef**: Defines a new function.
  - Grammar: `FunctionDef = "fn" name:Identifier "(" params:ParamList ")" "{" body:Block "}"`
- **FunctionCall**: Calls a function.
  - Grammar: `FunctionCall = name:Identifier "(" args:ArgList ")" | name:Identifier "(" ")"`
- **Return**: Returns a value from a function.
  - Grammar: `Return = "return" Expr`
- **If**: Conditional execution.
  - Grammar: `If = "if" condition:Expr then:Block "else" else:Block | "if" condition:Expr then:Block`

## Expressions
- **Expr**: The base rule for expressions, usually pointing to the lowest precedence operation (e.g., `LogicalOr`).
- **Logical**: Handles `&&` (AND) and `||` (OR) operations.
  - Grammar: `LogicalOr = LogicalAnd "||" LogicalOr | LogicalAnd`
- **Comparison**: Handles `==`, `!=`, `<`, `>` operations.
  - Grammar: `Comparison = Term Eq Term | Term Neq Term | ...`
- **Term**: Handles `+` and `-` operations.
  - Grammar: `Term = Factor Add Term | Factor Sub Term | Factor`
- **Factor**: Handles `*` and `/` operations.
  - Grammar: `Factor = Unary Mul Factor | Unary Div Factor | Unary`
- **Unary**: Handles unary operators like `!`.
  - Grammar: `Unary = UnaryOp Unary | Atom`

## Literals & Atoms
- **Literal**: Represents primitive values.
  - **Int**: `Int = [[0-9]+]`
  - **Float**: `Float = [[0-9]+\.[0-9]+]`
  - **String**: `String = ["[^\"]*"]`
  - **Bool**: `True = "true"`, `False = "false"`
- **Variable**: Represents an identifier.
  - Grammar: `Identifier = [[a-zA-Z_][a-zA-Z0-9_]*]`
- **ListNode**: Used for parameter and argument lists.
  - Grammar: `ParamList = Identifier "," params:ParamList | Identifier`

## Operators
Operators are defined as specific rules in the grammar:
- `Eq = [==]`
- `Neq = [!=]`
- `Lt = [<]`
- `Gt = [>]`
- `Add = [\+]`
- `Sub = [-]`
- `Mul = [\*]`
- `Div = [/]`
- `UnaryOp = [!]`
