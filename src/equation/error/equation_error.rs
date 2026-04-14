use super::super::EquationResultString;
use super::EquationErrorRenderable;
use super::types::EmptyEquationError;
use std::fmt::{Debug, Display, Formatter};

/// Errors that can occur while working with equations.
#[derive(Debug)]
pub enum EquationError {
	/// The input equation was empty.
	EmptyEquationError(EmptyEquationError),
}

impl From<EmptyEquationError> for EquationError {
	fn from(error: EmptyEquationError) -> Self {
		EquationError::EmptyEquationError(error)
	}
}

impl Display for EquationError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			EquationError::EmptyEquationError(error) => write!(f, "{}", error),
		}
	}
}

impl EquationErrorRenderable for EquationError {
	fn to_result_string(&self) -> EquationResultString {
		match self {
			EquationError::EmptyEquationError(empty_equation_error) => {
				empty_equation_error.to_result_string()
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn from_empty_equation_error_wraps_variant() {
		let equation_error: EquationError = EmptyEquationError.into();

		match equation_error {
			EquationError::EmptyEquationError(_) => {}
		}
	}

	#[test]
	fn display_delegates_to_inner_error() {
		let equation_error: EquationError = EquationError::from(EmptyEquationError);

		assert_eq!(equation_error.to_string(), "Equation cannot be empty");
	}

	#[test]
	fn to_result_string_delegates_to_inner_error() {
		let equation_error: EquationError = EquationError::from(EmptyEquationError);

		const EXPECTED_ERROR: &str = "\x1b[48;5;203m\x1b[38;5;255m\x1b[1m ERROR \x1b[0m \x1b[40m\
		\x1b[38;5;203mEmptyEquationError\x1b[0m: Equation cannot be empty";

		assert_eq!(
			equation_error.to_result_string().to_string(),
			EXPECTED_ERROR
		);
	}
}
