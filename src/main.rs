//! Command-line interface for the LaTeX Math crate.
//!
//! The binary accepts an equation as a command-line argument, parses it,
//! and prints:
//! - the parse result
//! - the token list
//! - the AST representation

use latex_math::equation::{Equation, EquationRenderable, EquationResult};
use std::env::args;

fn main() {
	let args: Vec<String> = args().collect();
	let output = render_output(&args[1]);
	print!("{output}");
}

fn render_output(input: &str) -> String {
	let equation_result: EquationResult = Equation::new(input);
	let equation_result_string: String = equation_result.to_string();
	let equation: Equation = equation_result.unwrap();

	format!(
		"\n{}\nTokens\n{}\nAST\n{}\n",
		equation_result_string,
		equation.to_token_name_list(),
		equation.to_ast_string()
	)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn render_output_includes_parse_result_tokens_and_ast() {
		let output: String = render_output("1+2");

		assert!(output.contains("Tokens"));
		assert!(output.contains("AST"));
	}
}
