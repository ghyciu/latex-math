use super::super::EquationResultString;

pub trait EquationErrorRenderable {
	fn to_result_string(&self) -> EquationResultString;
}