use super::super::EquationResultString;

/// Trait for equation errors that can be rendered as a result string.
pub trait EquationErrorRenderable {
	/// Converts the error into a displayable result string.
	fn to_result_string(&self) -> EquationResultString;
}