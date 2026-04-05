use crate::ast::{ASTNodeStringPrefix, ASTNodeRenderable, ASTNodeString};
use super::node_types::{ASTNodeNumber, ASTNodeBinary};

#[derive(Debug)]
pub enum ASTNode {
	Number(ASTNodeNumber),
	Binary(ASTNodeBinary)
}

impl ASTNodeRenderable for ASTNode {
	fn to_ast_node_string(&self, ast_node_prefix: ASTNodeStringPrefix) -> ASTNodeString {
		match self {
			ASTNode::Number(number) => number.to_ast_node_string(ast_node_prefix),
			ASTNode::Binary(binary) => binary.to_ast_node_string(ast_node_prefix)
		}
	}
}