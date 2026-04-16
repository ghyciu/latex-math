use super::super::EquationResultString;
use super::EquationErrorRenderable;
use super::types::{DelimiterMismatchError, EmptyEquationError};
use std::fmt::{Debug, Display, Formatter};

/// Errors that can occur while working with equations.
#[derive(Debug)]
pub enum EquationError {
	/// The input equation was empty.
	EmptyEquationError(EmptyEquationError),
	DelimiterMismatchError(DelimiterMismatchError),
}

impl From<EmptyEquationError> for EquationError {
	fn from(error: EmptyEquationError) -> Self {
		EquationError::EmptyEquationError(error)
	}
}

impl From<DelimiterMismatchError> for EquationError {
	fn from(error: DelimiterMismatchError) -> Self {
		EquationError::DelimiterMismatchError(error)
	}
}

impl Display for EquationError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			EquationError::EmptyEquationError(error) => write!(f, "{}", error),
			EquationError::DelimiterMismatchError(error) => write!(f, "{}", error),
		}
	}
}

impl EquationErrorRenderable for EquationError {
	fn to_result_string(&self) -> EquationResultString {
		match self {
			EquationError::EmptyEquationError(empty_equation_error) => {
				empty_equation_error.to_result_string()
			},
			EquationError::DelimiterMismatchError(delimiter_mismatch_error) => {
				delimiter_mismatch_error.to_result_string()
			},
		}
	}
}
