use crate::ast::{ASTNodeName, ASTNodeRenderable};
use crate::token::{TokenRenderable};
use crate::token::types::TokenNumber;

#[derive(Debug)]
pub struct ASTNodeNumber(TokenNumber);

impl ASTNodeNumber {
	pub fn new(number: TokenNumber) -> ASTNodeNumber {
		ASTNodeNumber(number)
	}
}

impl ASTNodeRenderable for ASTNodeNumber {
	fn get_name(&self) -> ASTNodeName {
		let node_name: String = self.0.get_name().to_string();
		ASTNodeName::new(node_name)
	}
}
