use std::fmt::Display;
use crate::ast::{ASTNode, ASTNodeRenderable, ASTNodeStringPrefix};

#[derive(Clone)]
pub struct ASTNodeString {
	prefix: ASTNodeStringPrefix,
	node: ASTNode,
}

impl ASTNodeString {
	pub fn new(prefix: ASTNodeStringPrefix, node: ASTNode) -> ASTNodeString {
		ASTNodeString {
			prefix,
			node
		}
	}
}

impl Display for ASTNodeString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self.node {
			ASTNode::Number(number) => {
				write!(f, "{}{}\n", self.prefix, number.get_name())?;
			},
			ASTNode::Binary(binary) => {
				let left_string: ASTNodeString = ASTNodeString::new(self.prefix.child(false), *binary.left.clone());
				let right_string: ASTNodeString = ASTNodeString::new(self.prefix.child(true), *binary.right.clone());
				write!(f, "{}{}\n{}{}", self.prefix, binary.get_name(), left_string, right_string)?;
			}
		}
		Ok({})
	}
}