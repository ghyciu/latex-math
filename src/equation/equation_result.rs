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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::equation::Equation;
	use crate::equation::error::types::EmptyEquationError;

	#[test]
	fn ok_displays_successful_equation_result() {
		let result: EquationResult = Equation::new("1+2");
		const EXPECTED_OK: &str = "\x1b[48;5;34m\x1b[38;5;255m\x1b[1m EQUATION \x1b[0m 1+2";

		assert_eq!(result.to_string(), EXPECTED_OK);
	}

	#[test]
	fn err_displays_error_equation_result() {
		let result: EquationResult = EquationResult::err(EmptyEquationError);
		const EXPECTED_ERROR: &str = "\x1b[48;5;203m\x1b[38;5;255m\x1b[1m ERROR \
		\x1b[0m \x1b[40m\x1b[38;5;203mEmptyEquationError\x1b[0m: Equation cannot be empty";

		assert_eq!(result.to_string(), EXPECTED_ERROR);
	}

	#[test]
	fn unwrap_returns_inner_equation_for_success_result() {
		let result: EquationResult = Equation::new("1+2");
		let equation: Equation = result.unwrap();
		const EXPECTED_OK: &str = "\x1b[48;5;34m\x1b[38;5;255m\x1b[1m EQUATION \x1b[0m 1+2";

		assert_eq!(equation.to_result_string().to_string(), EXPECTED_OK);
	}
}
