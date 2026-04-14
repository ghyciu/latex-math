use super::node_types::{ASTNodeBinary, ASTNodeNumber, ASTNodeUnary};
use crate::ast::{ASTNodeName, ASTNodeRenderable, ASTNodeString, ASTNodeStringPrefix};

/// A node in the abstract syntax tree for an expression.
///
/// This enum wraps the concrete node kinds used by the parser:
/// - numeric literals
/// - binary operator expressions
/// - unary operator expressions
#[derive(Debug)]
pub enum ASTNode {
	/// A numeric literal node.
	Number(ASTNodeNumber),

	/// A binary operator node.
	Binary(ASTNodeBinary),

	/// A unary operator node.
	Unary(ASTNodeUnary),
}

impl From<ASTNodeNumber> for ASTNode {
	fn from(number: ASTNodeNumber) -> ASTNode {
		ASTNode::Number(number)
	}
}

impl ASTNode {
	/// Converts the node into a formatted tree-like AST string.
	pub fn to_ast_node_string(&self) -> ASTNodeString<'_> {
		ASTNodeString::new(ASTNodeStringPrefix::new(), self)
	}
}

impl ASTNodeRenderable for ASTNode {
	/// Returns a human-readable name for the node.
	fn get_name(&self) -> ASTNodeName {
		match self {
			ASTNode::Number(number) => number.get_name(),
			ASTNode::Binary(binary) => binary.get_name(),
			ASTNode::Unary(unary) => unary.get_name(),
		}
	}
}
