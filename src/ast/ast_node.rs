use crate::ast::{ASTNodeRenderable, ASTNodeString, ASTNodeStringPrefix};
use super::node_types::{ASTNodeNumber, ASTNodeBinary};

#[derive(Debug, Clone)]
pub enum ASTNode {
	Number(ASTNodeNumber),
	Binary(ASTNodeBinary)
}

impl ASTNode {
	pub fn to_ast_node_string(&self) -> ASTNodeString {
		ASTNodeString::new(ASTNodeStringPrefix::new(), self.clone())
	}
}

impl ASTNodeRenderable for ASTNode {
	fn get_name(&self) -> String {
		match self {
			ASTNode::Number(number) => number.get_name(),
			ASTNode::Binary(binary) => binary.get_name()
		}
	}
}