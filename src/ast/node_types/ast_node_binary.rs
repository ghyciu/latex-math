use crate::ast::{ASTNode, ASTNodeRenderable};
use crate::token::{Token, TokenRenderable};

#[derive(Debug)]
pub struct ASTNodeBinary {
	left: Box<ASTNode>,
	operator: Token,
	right: Box<ASTNode>
}

impl ASTNodeBinary {
	pub fn new(left: Box<ASTNode>, operator: Token, right: Box<ASTNode>) -> ASTNodeBinary {
		ASTNodeBinary {
			left, operator, right
		}
	}

	pub fn get_left(&self) -> &ASTNode {
		&self.left
	}

	pub fn get_right(&self) -> &ASTNode {
		&self.right
	}
}

impl ASTNodeRenderable for ASTNodeBinary {
	fn get_name(&self) -> String {
		self.operator.as_token_string().to_string()
	}
}