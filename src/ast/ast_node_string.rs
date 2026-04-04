use std::fmt::Display;
use super::ast_node_prefix::ASTNodePrefix;

pub struct ASTNodeString(String);

impl ASTNodeString {
	pub fn new(string: String) -> ASTNodeString {
		ASTNodeString(string)
	}

	pub fn push(&mut self, string: ASTNodeString) {
		self.0 += string.get();
	}
	
	fn get(&self) -> &str {
		&self.0
	}
}

impl Display for ASTNodeString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}