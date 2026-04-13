use std::fmt::{Debug, Display, Formatter};
use std::error::Error;
use super::super::{EquationErrorRenderable};
use super::super::super::EquationResultString;

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