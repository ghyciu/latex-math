use std::fmt::{Display, Formatter};

pub struct ASTNodeString(String);

impl ASTNodeString {
	pub fn new(string: String) -> ASTNodeString {
		ASTNodeString(string)
	}
}

impl Display for ASTNodeString {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}\n", self.0)
	}
}