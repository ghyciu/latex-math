# latex-math

A Rust crate and command-line tool for parsing and rendering LaTeX-style math expressions.

## Features

- Parses math expressions from a string input
- Produces a parse result
- Lists tokens
- Renders an AST representation

## Usage

Run the binary and pass an equation as the first command-line argument:

`cargo run -- "1+2"`

## Examples

### Simple addition

`cargo run -- "1+2+3"`

This prints the parse result, token list, and AST for the expression.

```
EQUATION  1+2+3

Tokens
Number(1)
Operator(+)
Number(2)
Operator(+)
Number(3)

AST
BinaryOperator(+)
├── BinaryOperator(+)
│   ├── Number(1)
│   └── Number(2)
└── Number(3)

```

## Output

The program prints:

- the parse result
- the token list
- the AST representation

## Testing

Run the test suite with:

`cargo test`

## Project Structure

- `src/main.rs` — command-line entry point
- `src/lib.rs` — library entry point
- `src/equation/` — equation parsing and rendering
- `src/token/` — token parsing and token types
- `src/ast/` — AST parsing and AST types