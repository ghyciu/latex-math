use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::equation::EquationResultString;
use crate::equation::error::EquationErrorRenderable;

#[derive(Debug)]
pub struct DelimiterMismatchError;

impl Error for DelimiterMismatchError {}

impl Display for DelimiterMismatchError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "Equation cannot be empty")
	}
}

impl EquationErrorRenderable for DelimiterMismatchError {
	fn to_result_string(&self) -> EquationResultString {
		EquationResultString::err("DelimiterMismatchError", "Equation delimiter mismatch")
	}
}