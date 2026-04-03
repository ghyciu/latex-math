use crate::ast::ASTString;
use crate::token::TokenStringList;

pub trait EquationRenderable {
	fn as_token_string_list(&self) -> TokenStringList;
	fn as_ast_string(&self) -> ASTString;
}