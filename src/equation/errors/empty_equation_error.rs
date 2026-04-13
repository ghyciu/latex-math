use std::fmt::{Debug, Display, Formatter};
use std::error::Error;
use crate::equation::{EquationErrorRenderable, EquationErrorResultString};

#[derive(Debug)]
pub struct EmptyEquationError;

impl Error for EmptyEquationError {}

impl Display for EmptyEquationError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "Equation cannot be empty")
	}
}

impl EquationErrorRenderable for EmptyEquationError {
	fn to_error_result_string(&self) -> EquationErrorResultString {
		EquationErrorResultString::new("\n\x1b[1;91mEmptyEquationError:\x1b[0m Equation cannot be empty\n")
	}
}