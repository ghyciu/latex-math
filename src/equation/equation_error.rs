use std::fmt::{Debug, Display, Formatter};
use crate::equation::errors::EmptyEquationError;

#[derive(Debug)]
pub enum EquationError {
	EmptyEquationError(EmptyEquationError)
}

impl Display for EquationError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			EquationError::EmptyEquationError(error) => write!(f, "{}", error)
		}
	}
}