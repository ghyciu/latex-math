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
	pub fn new(left: Box<ASTNode>, operator: Token, right: Box<ASTNode>) -> ASTNodeBinary {
		ASTNodeBinary {
			left,
			operator,
			right,
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
