use std::fmt::Display;
use super::ASTNodeString;

pub struct ASTString(Vec<ASTNodeString>);

impl ASTString {
	pub fn new() -> ASTString {
		ASTString(Vec::new())
	}

	pub fn push(&mut self, ast_node_string: ASTNodeString) {
		self.0.push(ast_node_string);
	}
}

impl Display for ASTString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for ast_node_string in self.0.iter() {
			write!(f, "{}", ast_node_string)?;
		}
		Ok(())
	}
}