use crate::ast::ASTNodeRenderable;
use crate::token::TokenRenderable;
use crate::token::types::TokenNumber;

#[derive(Debug)]
pub struct ASTNodeNumber(TokenNumber);

impl ASTNodeNumber {
	pub fn new(number: TokenNumber) -> ASTNodeNumber {
		ASTNodeNumber(number)
	}
}

impl ASTNodeRenderable for ASTNodeNumber {
	fn get_name(&self) -> String {
		self.0.as_token_string().to_string()
	}
}
