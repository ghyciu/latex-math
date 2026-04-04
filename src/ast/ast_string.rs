use std::fmt::Display;
use crate::token::Token;
use super::{ASTNode, ASTNodePrefix, ASTNodeRenderable, ASTParser};

pub struct ASTString {
	root: ASTNode
}

impl ASTString {
	pub fn new(tokens: Vec<Token>) -> ASTString {
		let mut ast_parser: ASTParser = ASTParser::new(tokens);
		let root: ASTNode = ast_parser.parse();
		ASTString {
			root
		}
	}
}

impl Display for ASTString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.root.to_ast_node_string(ASTNodePrefix::new(), true))
	}
}