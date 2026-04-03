use crate::ast::{ASTNodePrefix, ASTNodeString};
use super::{Number, TokenString, TokenRenderable};

#[derive(Debug)]
pub enum Token {
	Number(Number)
}

impl TokenRenderable for Token {
	fn as_token_string(&self) -> TokenString {
		match self {
			Token::Number(number) => number.as_token_string()
		}
	}

	fn as_ast_node_string(&self, ast_node_prefix: ASTNodePrefix, is_last: bool) -> ASTNodeString {
		match self {
			Token::Number(number) => number.as_ast_node_string(ast_node_prefix, is_last)
		}
	}
}