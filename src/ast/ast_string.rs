use super::{ASTNode, ASTParser};
use crate::token::Token;
use std::fmt::Display;

pub struct ASTString(ASTNode);

impl ASTString {
	pub fn new(tokens: &Vec<Token>) -> ASTString {
		let mut ast_parser: ASTParser = ASTParser::new(tokens);
		let root: ASTNode = ast_parser.parse();
		ASTString(root)
	}
}

impl Display for ASTString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0.to_ast_node_string())?;
		Ok({})
	}
}
