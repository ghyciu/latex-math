use crate::ast::ASTString;
use crate::equation::EquationResultString;
use crate::token::TokenNameList;

pub trait EquationRenderable {
	fn to_result_string(&self) -> EquationResultString;
	fn to_token_name_list(&self) -> TokenNameList;
	fn to_ast_string(&self) -> ASTString;
}
