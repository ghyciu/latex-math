use crate::ast::{ASTNodePrefix, ASTNodeRenderable, ASTNodeString};
use crate::token::Number;

#[derive(Debug)]
pub struct ASTNodeNumber(Number);

impl ASTNodeNumber {
	pub fn new(number: Number) -> ASTNodeNumber {
		ASTNodeNumber(number)
	}
}

impl ASTNodeRenderable for ASTNodeNumber {
	fn as_ast_node_string(&self, prefix: ASTNodePrefix, is_last: bool) -> ASTNodeString {
		let connector: String = if prefix.is_empty() {
			String::from("")
		} else if is_last {
			String::from("└── ")
		} else {
			String::from("├── ")
		};
		ASTNodeString::new(format!("{}{}Number({})\n", prefix, connector, self.0.get()))
	}
}