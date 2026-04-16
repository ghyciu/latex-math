//! Command-line interface for the LaTeX Math crate.
//!
//! The binary accepts an equation as a command-line argument, parses it,
//! and prints:
//! - the parse result
//! - the token list
//! - the AST representation

use latex_math::equation::{Equation, EquationResult};
use std::env::args;

fn main() {
	let args: Vec<String> = args().collect();
	let output = render_output(&args[1]);
	print!("{output}");
}

fn render_output(input: &str) -> String {
	let equation_result: EquationResult = Equation::new(input);
	let equation_result_string: String = equation_result.to_string();

	format!(
		"\n{}",
		equation_result_string,
	)
}
