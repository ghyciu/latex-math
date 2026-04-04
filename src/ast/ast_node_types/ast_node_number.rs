use crate::token::Number;

#[derive(Debug)]
pub struct ASTNodeNumber(Number);

impl ASTNodeNumber {
	pub fn new(number: Number) -> ASTNodeNumber {
		ASTNodeNumber(number)
	}
}