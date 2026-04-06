use crate::ast::ASTString;
use crate::token::TokenStringList;

pub trait EquationRenderable {
	fn to_token_string_list(&self) -> TokenStringList;
	fn to_ast_string(&self) -> ASTString;
}
