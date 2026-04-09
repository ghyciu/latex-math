use super::node_types::{ASTNodeBinary, ASTNodeNumber, ASTNodeUnary};
use crate::ast::{ASTNodeName, ASTNodeRenderable, ASTNodeString, ASTNodeStringPrefix};

#[derive(Debug)]
pub enum ASTNode {
	Number(ASTNodeNumber),
	Binary(ASTNodeBinary),
	Unary(ASTNodeUnary)
}

impl ASTNode {
	pub fn to_ast_node_string(&self) -> ASTNodeString<'_> {
		ASTNodeString::new(ASTNodeStringPrefix::new(), self)
	}
}

impl ASTNodeRenderable for ASTNode {
	fn get_name(&self) -> ASTNodeName {
		match self {
			ASTNode::Number(number) => number.get_name(),
			ASTNode::Binary(binary) => binary.get_name(),
			ASTNode::Unary(unary) => unary.get_name(),
		}
	}
}
