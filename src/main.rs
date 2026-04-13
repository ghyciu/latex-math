use latex_math::equation::{Equation, EquationRenderable};
use std::env::args;

fn main() {
	let args: Vec<String> = args().collect();
	let equation: Equation = Equation::new(&args[1]).unwrap();
	print!("Tokens\n{}\n", equation.to_token_name_list());
	print!("AST\n{}\n", equation.to_ast_string());
}
