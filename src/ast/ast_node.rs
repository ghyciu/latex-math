use crate::ast::{ASTNodePrefix, ASTNodeRenderable, ASTNodeString};
use super::ast_node_types::{ASTNodeNumber, ASTNodeBinary};

#[derive(Debug)]
pub enum ASTNode {
	Number(ASTNodeNumber),
	Binary(ASTNodeBinary)
}

impl ASTNodeRenderable for ASTNode {
	fn as_ast_node_string(&self, ast_node_prefix: ASTNodePrefix, is_last: bool) -> ASTNodeString {
		match self {
			ASTNode::Number(number) => number.as_ast_node_string(ast_node_prefix, is_last),
			ASTNode::Binary(binary) => binary.as_ast_node_string(ast_node_prefix, is_last)
		}
	}
}