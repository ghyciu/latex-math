use super::EquationErrorResultString;

pub trait EquationErrorRenderable {
	fn to_error_result_string(&self) -> EquationErrorResultString;
}