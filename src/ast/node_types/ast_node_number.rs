use crate::ast::ASTNodeRenderable;
use crate::token::{Number, TokenRenderable};

#[derive(Debug, Clone)]
pub struct ASTNodeNumber(Number);

impl ASTNodeNumber {
	pub fn new(number: Number) -> ASTNodeNumber {
		ASTNodeNumber(number)
	}
}

impl ASTNodeRenderable for ASTNodeNumber {
	fn get_name(&self) -> String {
		self.0.as_token_string().to_string()
	}
}