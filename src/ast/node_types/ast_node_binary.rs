use crate::ast::{ASTNode, ASTNodeStringPrefix, ASTNodeRenderable, ASTNodeString};
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
	fn to_ast_node_string(&self, prefix: ASTNodeStringPrefix) -> ASTNodeString {
		let mut ast_node_string: ASTNodeString = ASTNodeString::new(prefix.clone(), String::from("Operator(+)"));
		let children = [&*self.left, &*self.right];
		for (i, child) in children.iter().enumerate() {
			let is_last: bool = i == children.len() - 1;
			let child_prefix: ASTNodeStringPrefix = prefix.child(is_last);
			ast_node_string.push(child.to_ast_node_string(child_prefix));
		}

		ast_node_string
	}
}