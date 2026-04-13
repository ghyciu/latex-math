use std::fmt::Display;

pub struct EquationErrorResultString(String);

impl EquationErrorResultString {
	pub fn new<T: Into<String>>(string: T) -> EquationErrorResultString {
		EquationErrorResultString(string.into())
	}
}

impl Display for EquationErrorResultString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}