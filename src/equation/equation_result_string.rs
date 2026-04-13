use std::fmt::Display;
use crate::token::{Token, TokenRenderable};

/// A formatted string used to display an equation or an equation error.
pub struct EquationResultString(String);

impl EquationResultString {

	fn new(string: String) -> EquationResultString {
		EquationResultString(string)
	}

	/// Creates a formatted success string from a token list.
	pub fn ok(tokens: &Vec<Token>) -> EquationResultString {
		const EQUATION_LABEL: &'static str = "\x1b[48;5;34m\x1b[38;5;255m\x1b[1m EQUATION \x1b[0m";
		let result_string: String = tokens.iter().map(|token| token.get_value().to_string()).collect();
		EquationResultString::new(format!("{EQUATION_LABEL} {}", result_string))
	}

	/// Creates a formatted error string from an error name and description.
	pub fn err<T: Into<String>>(name: T, description: T) -> EquationResultString {
		const ERROR_LABEL: &'static str = "\x1b[48;5;203m\x1b[38;5;255m\x1b[1m ERROR \x1b[0m";
		let name: String = format!("\x1b[40m\x1b[38;5;203m{}\x1b[0m", name.into());
		let result_string: String = format!("{ERROR_LABEL} {name}: {}", description.into());
		EquationResultString::new(format!("{}", result_string))
	}
}

impl Display for EquationResultString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			write!(f, "{}", self.0)
	}
}
