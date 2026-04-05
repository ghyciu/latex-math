use crate::ast::{ASTNode, ASTNodeRenderable};
use crate::token::{Token, TokenRenderable};

#[derive(Debug, Clone)]
pub struct ASTNodeBinary {
	pub(crate) left: Box<ASTNode>,
	operator: Token,
	pub(crate) right: Box<ASTNode>
}

impl ASTNodeBinary {
	pub fn new(left: Box<ASTNode>, operator: Token, right: Box<ASTNode>) -> ASTNodeBinary {
		ASTNodeBinary {
			left, operator, right
		}
	}
}

impl ASTNodeRenderable for ASTNodeBinary {
	fn get_name(&self) -> String {
		self.operator.as_token_string().to_string()
	}
}