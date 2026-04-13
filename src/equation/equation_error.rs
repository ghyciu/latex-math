use std::fmt::{Debug, Display, Formatter};
use crate::equation::{EquationErrorRenderable, EquationErrorResultString};
use crate::equation::errors::EmptyEquationError;

#[derive(Debug)]
pub enum EquationError {
	EmptyEquationError(EmptyEquationError)
}

impl From<EmptyEquationError> for EquationError {
	fn from(error: EmptyEquationError) -> Self {
		EquationError::EmptyEquationError(error)
	}
}

impl Display for EquationError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			EquationError::EmptyEquationError(error) => write!(f, "{}", error)
		}
	}
}

impl EquationErrorRenderable for EquationError {
	fn to_error_result_string(&self) -> EquationErrorResultString {
			match self {
				EquationError::EmptyEquationError(empty_equation_error) => {
					empty_equation_error.to_error_result_string()
				}
			}
		}
}