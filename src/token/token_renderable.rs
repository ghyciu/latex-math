use crate::ast::{ASTNodePrefix, ASTNodeString};
use super::TokenString;

pub trait TokenRenderable {
	fn as_token_string(&self) -> TokenString;
	fn as_ast_node_string(&self, ast_node_prefix: ASTNodePrefix, is_last: bool) -> ASTNodeString;
}