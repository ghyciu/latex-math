use crate::ast::{ASTNodePrefix, ASTNodeString};
use super::{TokenRenderable, TokenString};

#[derive(Debug)]
pub struct Number(String);

impl Number {
	pub fn new(number: String) -> Number {
		Number(number)
	}

	fn get(&self) -> &String {
		&self.0
	}
}

impl TokenRenderable for Number {
	fn as_token_string(&self) -> TokenString {
		TokenString::new(format!("Number({})", self.get()))
	}
	fn as_ast_node_string(&self, ast_node_prefix: ASTNodePrefix, is_last: bool) -> ASTNodeString {
		let connector: String = if ast_node_prefix.is_empty() {
			String::from("")
		} else if is_last {
			String::from("└─")
		} else {
			String::from("├─")
		};
		ASTNodeString::new(format!("{}{}Number({})", ast_node_prefix, connector, self.get()))
	}
}