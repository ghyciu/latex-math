use std::fmt::Display;

pub struct ASTNodeName(String);

impl ASTNodeName {
	pub fn new<T: Into<String>>(name: T) -> ASTNodeName {
		ASTNodeName(name.into())
	}
}

impl Display for ASTNodeName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			write!(f, "{}", self.0)
		}
}