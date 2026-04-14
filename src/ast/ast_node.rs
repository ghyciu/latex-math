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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ast::ASTNodeRenderable;
	use crate::ast::node_types::{ASTNodeBinary, ASTNodeNumber, ASTNodeUnary};
	use crate::token::types::{TokenOperator, TokenOperatorType};

	#[test]
	fn from_number_creates_number_node() {
		let node = ASTNode::from(ASTNodeNumber::new("1"));

		assert_eq!(node.get_name().to_string(), "Number(1)");
	}

	#[test]
	fn get_name_delegates_to_binary_node() {
		let left = ASTNodeNumber::new("1");
		let right = ASTNodeNumber::new("2");
		let binary = ASTNodeBinary::new(left, TokenOperator::new(TokenOperatorType::Add), right);
		let node = ASTNode::Binary(binary);

		assert_eq!(node.get_name().to_string(), "BinaryOperator(+)");
	}

	#[test]
	fn get_name_delegates_to_unary_node() {
		let operand = ASTNodeNumber::new("1");
		let unary = ASTNodeUnary::new(TokenOperator::new(TokenOperatorType::Add), operand);
		let node = ASTNode::Unary(unary);

		assert_eq!(node.get_name().to_string(), "UnaryOperator(+)");
	}

	#[test]
	fn to_ast_node_string_formats_number_node() {
		let node = ASTNode::from(ASTNodeNumber::new("1"));

		assert_eq!(node.to_ast_node_string().to_string(), "Number(1)\n");
	}

	#[test]
	fn to_ast_node_string_formats_binary_node() {
		let left = ASTNodeNumber::new("1");
		let right = ASTNodeNumber::new("2");
		let binary = ASTNodeBinary::new(left, TokenOperator::new(TokenOperatorType::Add), right);
		let node = ASTNode::Binary(binary);

		assert_eq!(
			node.to_ast_node_string().to_string(),
			"BinaryOperator(+)\n├── Number(1)\n└── Number(2)\n"
		);
	}

	#[test]
	fn to_ast_node_string_formats_unary_node() {
		let operand = ASTNodeNumber::new("1");
		let unary = ASTNodeUnary::new(TokenOperator::new(TokenOperatorType::Add), operand);
		let node = ASTNode::Unary(unary);

		assert_eq!(
			node.to_ast_node_string().to_string(),
			"UnaryOperator(+)\n└── Number(1)\n"
		);
	}
}
