use crate::ast::ASTString;
use crate::equation::EquationResultString;
use crate::token::TokenNameList;

pub trait EquationRenderable {
	/// Trait for types that can be rendered in equation-related formats.
	fn to_result_string(&self) -> EquationResultString;

	/// Returns the token names for the equation.
	fn to_token_name_list(&self) -> TokenNameList;

	/// Returns the AST representation of the equation.
	fn to_ast_string(&self) -> ASTString;
}
