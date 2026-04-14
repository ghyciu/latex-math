use super::error::{EquationError, EquationErrorRenderable};
use super::{Equation, EquationRenderable};
use std::fmt::{Display, Formatter};

/// The result of parsing or constructing an equation.
///
/// This wraps either a successfully parsed [`Equation`] or an error.
pub struct EquationResult {
	result: Result<Equation, EquationError>,
}

impl EquationResult {
	/// Creates a successful equation result.
	pub fn ok(equation: Equation) -> EquationResult {
		EquationResult {
			result: Ok(equation),
		}
	}

	/// Creates an error equation result.
	pub fn err<E: Into<EquationError>>(error: E) -> EquationResult {
		EquationResult {
			result: Err(error.into()),
		}
	}

	/// Unwraps the inner equation, panicking if the result is an error.
	pub fn unwrap(self) -> Equation {
		self.result.unwrap()
	}
}

impl Display for EquationResult {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self.result {
			Ok(ref equation) => write!(f, "{}", equation.to_result_string()),
			Err(ref error) => write!(f, "{}", error.to_result_string()),
		}
	}
}
