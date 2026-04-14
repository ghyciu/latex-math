use crate::ast::{ASTNode, ASTNodeRenderable, ASTNodeStringPrefix};
use std::fmt::Display;

/// A formatted string representation of an AST subtree.
///
/// The output is rendered in a tree-like layout using prefixes and branches.
#[derive(Clone)]
pub struct ASTNodeString<'a> {
	prefix: ASTNodeStringPrefix,
	node: &'a ASTNode,
}

impl<'a> ASTNodeString<'a> {
	/// Creates a new AST string renderer for a node.
	pub fn new(prefix: ASTNodeStringPrefix, node: &'a ASTNode) -> ASTNodeString<'a> {
		ASTNodeString { prefix, node }
	}
}

impl<'a> Display for ASTNodeString<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self.node {
			ASTNode::Number(number) => {
				write!(f, "{}{}\n", self.prefix, number.get_name())
					.expect("Unable to generate ASTNodeNumber");
			}
			ASTNode::Binary(binary) => {
				let left_string: ASTNodeString =
					ASTNodeString::new(self.prefix.child(false), binary.get_left());
				let right_string: ASTNodeString =
					ASTNodeString::new(self.prefix.child(true), binary.get_right());
				write!(
					f,
					"{}{}\n{}{}",
					self.prefix,
					binary.get_name(),
					left_string,
					right_string
				)
				.expect("Unable to generate ASTNodeBinary");
			}
			ASTNode::Unary(unary) => {
				let child_string: ASTNodeString =
					ASTNodeString::new(self.prefix.child(true), unary.get_child());
				write!(f, "{}{}\n{}", self.prefix, unary.get_name(), child_string)
					.expect("Unable to generate ASTNodeUnary");
			}
		}
		Ok({})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ast::node_types::{ASTNodeBinary, ASTNodeNumber, ASTNodeUnary};
	use crate::token::types::{TokenOperator, TokenOperatorType};

	#[test]
	fn formats_number_without_prefix() {
		let node = ASTNode::from(ASTNodeNumber::new("42"));

		let rendered = ASTNodeString::new(ASTNodeStringPrefix::new(), &node).to_string();

		assert_eq!(rendered, "Number(42)\n");
	}

	#[test]
	fn formats_binary_with_nested_prefixes() {
		let left = ASTNodeNumber::new("1");
		let right = ASTNodeNumber::new("2");
		let binary = ASTNodeBinary::new(left, TokenOperator::new(TokenOperatorType::Add), right);
		let node = ASTNode::Binary(binary);

		let rendered = ASTNodeString::new(ASTNodeStringPrefix::new(), &node).to_string();

		assert_eq!(
			rendered,
			"BinaryOperator(+)\n├── Number(1)\n└── Number(2)\n"
		);
	}

	#[test]
	fn formats_unary_with_child_prefix() {
		let operand = ASTNodeNumber::new("1");
		let unary = ASTNodeUnary::new(TokenOperator::new(TokenOperatorType::Add), operand);
		let node = ASTNode::Unary(unary);

		let rendered = ASTNodeString::new(ASTNodeStringPrefix::new(), &node).to_string();

		assert_eq!(rendered, "UnaryOperator(+)\n└── Number(1)\n");
	}

	#[test]
	fn formats_nested_binary_tree() {
		let inner_left = ASTNodeNumber::new("1");
		let inner_right = ASTNodeNumber::new("2");
		let inner = ASTNodeBinary::new(
			inner_left,
			TokenOperator::new(TokenOperatorType::Add),
			inner_right,
		);

		let outer_right = ASTNodeNumber::new("3");
		let outer = ASTNodeBinary::new(
			ASTNode::Binary(inner),
			TokenOperator::new(TokenOperatorType::Add),
			outer_right,
		);
		let node = ASTNode::Binary(outer);

		let rendered = ASTNodeString::new(ASTNodeStringPrefix::new(), &node).to_string();

		assert_eq!(
			rendered,
			"BinaryOperator(+)\n├── BinaryOperator(+)\n│   ├── Number(1)\n│   └── Number(2)\n└── Number(3)\n"
		);
	}
}
