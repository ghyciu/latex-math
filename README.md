# latex-math

[![](https://img.shields.io/crates/v/latex-math?style=for-the-badge&cacheSeconds=3600)](https://crates.io/crates/latex-math)
[![](https://img.shields.io/docsrs/latex-math?style=for-the-badge&cacheSeconds=3600)](https://docs.rs/latex-math/0.1.1/latex_math/)
![](https://img.shields.io/crates/l/latex-math?style=for-the-badge&cacheSeconds=3600)

[![](https://img.shields.io/github/stars/ghyciu/latex-math?style=for-the-badge&cacheSeconds=3600)](https://github.com/ghyciu/latex-math)
![](https://img.shields.io/crates/d/latex-math?style=for-the-badge&cacheSeconds=3600)

A Rust crate and command-line tool for parsing and rendering LaTeX-style math expressions.

## Changelog

See the full changelog in [CHANGELOG.md](CHANGELOG.md).

## Documentation

See the full documentation in [docs.rs](https://docs.rs/latex-math).

## Repository

See the full repository in [GitHub](https://github.com/ghyciu/latex-math).

## Installation

```cargo install latex-math```

Running the above command will install the `latex-math` binary in your system's PATH.

```cargo add latex-math```

Running the above command will add the `latex-math` crate to your `Cargo.toml` file. You may also add `latex-math = $VERSION_NUMBER` to your `Cargo.toml` file to pin the version of the crate.

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

## Support

This crate is simply a passion project and my way of learning Rust. If you like this project or would like to contribute in the future, please consider starring **[this repository](https://github.com/ghyciu/latex-math)** on GitHub. You may also consider visiting **[my profile](https://github.com/ghyciu)** on GitHub and following me for my other projects or to keep updated with the latest releases. Thank you very much for your support and your continued interest in the project keeps me going.

## License



## Contributing