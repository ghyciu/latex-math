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
