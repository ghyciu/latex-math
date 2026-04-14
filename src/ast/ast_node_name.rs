use std::fmt::Display;

/// A human-readable name for an AST node.
pub struct ASTNodeName(String);

impl ASTNodeName {
	/// Creates a new AST node name wrapper.
	pub fn new<T: Into<String>>(name: T) -> ASTNodeName {
		ASTNodeName(name.into())
	}
}

impl Display for ASTNodeName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_stores_string_value() {
		let name = ASTNodeName::new("Number(1)");

		assert_eq!(name.to_string(), "Number(1)");
	}
}