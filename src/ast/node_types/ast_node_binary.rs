use crate::ast::{ASTNode, ASTNodeName, ASTNodeRenderable};
use crate::token::{Token, TokenRenderable};

/// An AST node representing a binary operator expression.
///
/// A binary node stores:
/// - The left operand
/// - The operator token
/// - The right operand
#[derive(Debug)]
pub struct ASTNodeBinary {
	left: Box<ASTNode>,
	operator: Token,
	right: Box<ASTNode>,
}

impl ASTNodeBinary {
	/// Creates a new binary AST node.
	///
	/// # Parameters
	///
	/// - `left` - the left-hand operand
	/// - `operator` - the operator token
	/// - `right` - the right-hand operand
	///
	/// # Returns
	///
	/// A new [`ASTNodeBinary`] instance.
	pub fn new<L, T, R>(left: L, operator: T, right: R) -> ASTNodeBinary
	where L: Into<ASTNode>, T: Into<Token>, R: Into<ASTNode> {
		ASTNodeBinary {
			left: Box::new(left.into()),
			operator: operator.into(),
			right: Box::new(right.into())
		}
	}

	/// Returns a reference to the left child node.
	pub fn get_left(&self) -> &ASTNode {
		&self.left
	}

	/// Returns a reference to the right child node.
	pub fn get_right(&self) -> &ASTNode {
		&self.right
	}
}

impl ASTNodeRenderable for ASTNodeBinary {
	/// Returns the display name for this node.
	///
	/// The generated name has the form:
	/// `BinaryOperator(<operator>)`
	fn get_name(&self) -> ASTNodeName {
		let node_name: String = format!("BinaryOperator({})", self.operator.get_value());
		ASTNodeName::new(node_name)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ast::ASTNodeRenderable;
	use crate::ast::node_types::ASTNodeNumber;
	use crate::token::types::{TokenNumber, TokenOperator, TokenOperatorType};

	#[test]
	fn get_name_formats_binary_operator() {
		let left = ASTNodeNumber::new(TokenNumber::new("1"));
		let right = ASTNodeNumber::new(TokenNumber::new("2"));
		let node = ASTNodeBinary::new(left, TokenOperator::new(TokenOperatorType::Add), right);
		let name = node.get_name();

		assert_eq!(name.to_string(), "BinaryOperator(+)");
	}

	#[test]
	fn get_left_and_right_return_children() {
		let left = ASTNodeNumber::new(TokenNumber::new("1"));
		let right = ASTNodeNumber::new(TokenNumber::new("2"));
		let node = ASTNodeBinary::new(left, TokenOperator::new(TokenOperatorType::Add), right);

		assert_eq!(node.get_left().get_name().to_string(), "Number(1)");
		assert_eq!(node.get_right().get_name().to_string(), "Number(2)");
	}
}
