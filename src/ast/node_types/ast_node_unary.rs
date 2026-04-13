use crate::ast::{ASTNode, ASTNodeName, ASTNodeRenderable};
use crate::token::{Token, TokenRenderable};

#[derive(Debug)]
/// An AST node representing a unary operator expression.
///
/// A unary node stores:
/// - the operator token
/// - the operand node
pub struct ASTNodeUnary {
	operator: Token,
	operand: Box<ASTNode>,
}

impl ASTNodeUnary {
	/// Creates a new unary AST node.
	///
	/// # Parameters
	///
	/// - `operator` - the operator token
	/// - `operand` - the operand node
	///
	/// # Returns
	///
	/// A new [`ASTNodeUnary`] instance.
	pub fn new(operator: Token, operand: Box<ASTNode>) -> ASTNodeUnary {
		ASTNodeUnary { operator, operand }
	}

	/// Returns a reference to the operand node.
	pub fn get_child(&self) -> &ASTNode {
		&self.operand
	}
}

impl ASTNodeRenderable for ASTNodeUnary {
	/// Returns the display name for this node.
	///
	/// The generated name has the form:
	/// `UnaryOperator(<operator>)`
	fn get_name(&self) -> ASTNodeName {
		let node_name: String = format!("UnaryOperator({})", self.operator.get_value());
		ASTNodeName::new(node_name)
	}
}