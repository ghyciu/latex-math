use super::super::super::EquationResultString;
use super::super::EquationErrorRenderable;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Error returned when an equation input is empty.
#[derive(Debug)]
pub struct EmptyEquationError;

impl Error for EmptyEquationError {}

impl Display for EmptyEquationError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "Equation cannot be empty")
	}
}

impl EquationErrorRenderable for EmptyEquationError {
	fn to_result_string(&self) -> EquationResultString {
		EquationResultString::err("EmptyEquationError", "Equation cannot be empty")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn display_formats_empty_equation_message() {
		let error = EmptyEquationError;

		assert_eq!(error.to_string(), "Equation cannot be empty");
	}

	#[test]
	fn to_result_string_formats_empty_equation_error() {
		let error = EmptyEquationError;

		const EXPECTED_ERROR: &str = "\x1b[48;5;203m\x1b[38;5;255m\x1b[1m ERROR \x1b[0m \x1b[40m\
		\x1b[38;5;203mEmptyEquationError\x1b[0m: Equation cannot be empty";

		assert_eq!(error.to_result_string().to_string(), EXPECTED_ERROR);
	}
}
