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
	let equation_result: EquationResult = Equation::new(&args[1]);
	print!("\n{}\n", equation_result);
	let equation: Equation = equation_result.unwrap();
	print!("Tokens\n{}\n", equation.to_token_name_list());
	print!("AST\n{}\n", equation.to_ast_string());
}
