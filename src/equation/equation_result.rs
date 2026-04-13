use std::fmt::{Display, Formatter};
use super::{Equation, EquationError, EquationErrorRenderable};

pub struct EquationResult {
	result: Result<Equation, EquationError>
}

impl EquationResult {
	pub fn ok(equation: Equation) -> EquationResult {
		EquationResult { result: Ok(equation) }
	}

	pub fn err<E: Into<EquationError>>(error: E) -> EquationResult {
		EquationResult { result: Err(error.into()) }
	}

	pub fn unwrap(self) -> Equation {
		self.result.unwrap()
	}
}

impl Display for EquationResult {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self.result {
			Ok(_) => write!(f, "Equation is ok"),
			Err(ref error) => write!(f, "{}", error.to_error_result_string())
		}
	}
}