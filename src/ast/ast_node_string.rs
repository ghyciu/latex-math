use std::fmt::Display;
use crate::ast::ASTNodePrefix;

pub struct ASTNodeString {
	prefix: ASTNodePrefix,
	is_last: bool,
	string: String,
}

impl ASTNodeString {
	pub fn new(prefix: ASTNodePrefix, is_last: bool, string: String) -> ASTNodeString {
		ASTNodeString {
			prefix, is_last,
			string: format!("{}\n", string)
		}
	}

	pub fn push(&mut self, node: ASTNodeString) {
		self.string += &node.to_string();
	}
}

impl Display for ASTNodeString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let connector: String = if self.prefix.is_empty() {
			String::from("")
		} else if self.is_last {
			String::from("└── ")
		} else {
			String::from("├── ")
		};
		write!(f, "{}{}{}", self.prefix, connector, self.string)
	}
}