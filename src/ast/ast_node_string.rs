use std::fmt::Display;
use crate::ast::ASTNodeStringPrefix;

pub struct ASTNodeString {
	prefix: ASTNodeStringPrefix,
	string: String,
}

impl ASTNodeString {
	pub fn new(prefix: ASTNodeStringPrefix, string: String) -> ASTNodeString {
		ASTNodeString {
			prefix,
			string: format!("{}\n", string)
		}
	}

	pub fn push(&mut self, node: ASTNodeString) {
		self.string += &node.to_string();
	}
}

impl Display for ASTNodeString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{}", self.prefix, self.string)
	}
}