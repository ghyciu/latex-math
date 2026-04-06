use super::node_types::{ASTNodeBinary, ASTNodeNumber};
use crate::ast::{ASTNodeRenderable, ASTNodeString, ASTNodeStringPrefix};

#[derive(Debug)]
pub enum ASTNode {
	Number(ASTNodeNumber),
	Binary(ASTNodeBinary),
}

impl ASTNode {
	pub fn to_ast_node_string(&self) -> ASTNodeString<'_> {
		ASTNodeString::new(ASTNodeStringPrefix::new(), self)
	}
}

impl ASTNodeRenderable for ASTNode {
	fn get_name(&self) -> String {
		match self {
			ASTNode::Number(number) => number.get_name(),
			ASTNode::Binary(binary) => binary.get_name(),
		}
	}
}
