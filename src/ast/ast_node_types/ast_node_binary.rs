use crate::ast::ASTNode;
use crate::token::Token;

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
}