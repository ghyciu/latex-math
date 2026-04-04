use crate::ast::{ASTNode, ASTNodePrefix, ASTNodeRenderable, ASTNodeString};
use crate::token::Token;

#[derive(Debug)]
pub struct ASTNodeBinary {
	left: Box<ASTNode>,
	operator: Token,
	right: Box<ASTNode>
}

impl ASTNodeBinary {
	pub fn new(left: Box<ASTNode>, operator: Token, right: Box<ASTNode>) -> ASTNodeBinary {
		ASTNodeBinary {
			left, operator, right
		}
	}
}

impl ASTNodeRenderable for ASTNodeBinary {
	fn as_ast_node_string(&self, prefix: ASTNodePrefix, is_last: bool) -> ASTNodeString {
		let connector: String = if prefix.is_empty() {
			String::from("")
		} else if is_last {
			String::from("└── ")
		} else {
			String::from("├── ")
		};
		let mut ast_node_string: ASTNodeString = ASTNodeString::new(format!("{}{}Operator(+)\n", prefix, connector));
		let children = [&*self.left, &*self.right];

		for (i, child) in children.iter().enumerate() {
			let is_last: bool = i == children.len() - 1;
			let child_prefix: ASTNodePrefix = prefix.child(is_last);
			ast_node_string.push(child.as_ast_node_string(child_prefix, is_last));
		}

		ast_node_string
	}
}