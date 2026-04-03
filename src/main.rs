use std::env::args;
use latex_equation::equation::Equation;

fn main() {
	let args: Vec<String> = args().collect();
	let equation: Equation = Equation::new(args[1].clone());
	equation.print_tokens();
}