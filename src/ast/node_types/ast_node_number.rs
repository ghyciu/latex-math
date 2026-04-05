use crate::ast::{ASTNodeStringPrefix, ASTNodeRenderable, ASTNodeString};
use crate::token::Number;

#[derive(Debug)]
pub struct ASTNodeNumber(Number);

impl ASTNodeNumber {
	pub fn new(number: Number) -> ASTNodeNumber {
		ASTNodeNumber(number)
	}
}

impl ASTNodeRenderable for ASTNodeNumber {
	fn to_ast_node_string(&self, prefix: ASTNodeStringPrefix) -> ASTNodeString {
		ASTNodeString::new(prefix.clone(), format!("Number({})", self.0.get()))
	}
}