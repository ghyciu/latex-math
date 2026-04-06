use latex_equation::equation::{Equation, EquationRenderable};
use std::env::args;

fn main() {
	let args: Vec<String> = args().collect();
	let equation: Equation = Equation::new(args[1].clone());
	print!("Tokens\n{}\n", equation.to_token_string_list());
	print!("AST\n{}\n", equation.to_ast_string());
}
