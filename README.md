# Multilang

Multilang is a dynamic language runtime that allows you to define languages using a grammar and execute them instantly.

## Features

- **Dynamic Grammar**: Define languages using a simple EBNF-like syntax.
- **Extensible AST**: Built-in nodes for `Program`, `Print`, `Return`, `Int`.
- **Instant Execution**: No compilation step required for the defined language.

## Documentation

- [**Nodes**](docs/nodes.md): List of available AST nodes and their grammar definitions.
- [**Example**](docs/example.md): A complete example of a grammar and code.
- [**Roadmap**](docs/roadmap.md): Planned features and improvements.
- [**Standards**](docs/standards.md): Development standards and best practices.

## Getting Started
To run the project:
```bash
cargo run
```
To run tests:
```bash
cargo test
```

### Running the Interpreter

You can run the interpreter by providing a grammar file and a code file as arguments.

```bash
cargo run -- <grammar_file> <code_file>
```

**Example:**

Using the example resources provided in the `tests/resources` directory:

```bash
cargo run -- tests/resources/standard/grammar.mlg tests/resources/standard/two_sum.mlc
```

The `tests/resources` directory contains examples of different language grammars ("standard", "wordy", "cryptic") and corresponding code files (`two_sum.mlc`, `palindrome.mlc`, `fizzbuzz.mlc`) that demonstrate the flexibility of Multilang.
You can define your language grammar using a string. The format is:

```
RuleName = Pattern1 Pattern2 ...
```

### Rules

- **Literal**: Enclosed in double quotes, e.g., `"print"`. Matches exact text.
- **Regex**: Enclosed in brackets, e.g., `[[0-9]+]`. Matches a regular expression.
- **Rule Reference**: The name of another rule, e.g., `Stmt`.
- **Sequence**: Space-separated patterns are matched in order.
- **Alternatives**: Define the same rule multiple times to create alternatives.
- **Repetition**: Append `*` to a rule name to match zero or more times (e.g., `Stmt*`).

### Example

```
Program = Stmt*
Stmt = Print
Stmt = Return
Print = "print" Int
Return = "return" Int
Int = [[0-9]+]
```

This grammar defines a program as a sequence of statements. A statement can be a `Print` or `Return` command. `Int` matches one or more digits.

## Built-in Nodes

The parser maps specific rule names to built-in AST nodes:

- `Program`: Executes children sequentially.
- `Print`: Prints the value of its expression.
- `Return`: Returns the value of its expression.
- `Int`: Parses the matched text as an integer.
- `Term`: Handles addition (`Add`) and subtraction (`Sub`).
- `Factor`: Handles multiplication (`Mul`) and division (`Div`).
- `If`: Handles conditional logic (`IfElse`, `IfThen`).
- `Boolean`: Represents `true` or `false` values.

## Development Standards

### Naming Conventions

- **Rust Code**: Follow standard Rust naming conventions.
    - Structs/Enums: `CamelCase`
    - Functions/Variables/Modules: `snake_case`
- **Grammar Rules**: Use `CamelCase` for rule names (e.g., `Program`, `Stmt`, `IfElse`).

### Testing

- **Unit Tests**: Place unit tests in the same file as the code they test, within a `mod tests` module.
- **Integration Tests**: Place integration tests in the `tests/` directory.
    - `print_test.rs`: Tests for `Print` functionality.
    - `output_test.rs`: Tests for program return values.
    - `arithmetic_test.rs`: Tests for arithmetic operations and precedence.
    - `control_flow_test.rs`: Tests for `If/Else` logic.

