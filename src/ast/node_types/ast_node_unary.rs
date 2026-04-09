use crate::ast::{ASTNode, ASTNodeName, ASTNodeRenderable};
use crate::token::{Token, TokenRenderable};

#[derive(Debug)]
pub struct ASTNodeUnary {
	operator: Token,
	operand: Box<ASTNode>,
}

impl ASTNodeUnary {
	pub fn new(operator: Token, operand: Box<ASTNode>) -> ASTNodeUnary {
		ASTNodeUnary { operator, operand }
	}

	pub fn get_child(&self) -> &ASTNode {
		&self.operand
	}
}

impl ASTNodeRenderable for ASTNodeUnary {
	fn get_name(&self) -> ASTNodeName {
		let node_name: String = format!("UnaryOperator({})", self.operator.get_value());
		ASTNodeName::new(node_name)
	}
}