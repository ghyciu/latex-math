use std::fmt::{Debug, Display, Formatter};
use std::error::Error;

#[derive(Debug)]
pub struct EmptyEquationError;

impl Error for EmptyEquationError {}

impl Display for EmptyEquationError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "Equation cannot be empty")
	}
}