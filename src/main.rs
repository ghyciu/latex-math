use std::env::args;
use latex_equation::equation::{Equation, EquationRenderable};

fn main() {
	let args: Vec<String> = args().collect();
	let equation: Equation = Equation::new(args[1].clone());
	print!("Token List\n{}\n", equation.as_token_string_list());
	print!("Abstract Syntax Tree\n{}\n", equation.as_ast_string());
}