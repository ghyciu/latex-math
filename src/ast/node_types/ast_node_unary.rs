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
	pub fn new<T: Into<Token>>(operator: T, operand: Box<ASTNode>) -> ASTNodeUnary {
		ASTNodeUnary {
			operator: operator.into(),
			operand,
		}
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ast::ASTNodeRenderable;
	use crate::ast::node_types::ASTNodeNumber;
	use crate::token::types::{TokenNumber, TokenOperator, TokenOperatorType};

	#[test]
	fn get_name_formats_unary_operator() {
		let operand = Box::new(ASTNode::Number(ASTNodeNumber::new(TokenNumber::new("1"))));
		let node = ASTNodeUnary::new(TokenOperator::new(TokenOperatorType::Add), operand);
		let name = node.get_name();

		assert_eq!(name.to_string(), "UnaryOperator(+)");
	}

	#[test]
	fn get_child_returns_operand() {
		let operand = Box::new(ASTNode::Number(ASTNodeNumber::new(TokenNumber::new("1"))));
		let node = ASTNodeUnary::new(TokenOperator::new(TokenOperatorType::Add), operand);

		assert_eq!(node.get_child().get_name().to_string(), "Number(1)");
	}
}
