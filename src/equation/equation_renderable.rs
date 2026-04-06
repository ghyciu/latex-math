use crate::ast::ASTString;
use crate::token::TokenListString;

pub trait EquationRenderable {
	fn to_token_string_list(&self) -> TokenListString;
	fn to_ast_string(&self) -> ASTString;
}
