use crate::ast::{ASTNodeName, ASTNodeRenderable};
use crate::token::{TokenRenderable};
use crate::token::types::TokenNumber;

/// An AST node representing a numeric literal.
#[derive(Debug)]
pub struct ASTNodeNumber(TokenNumber);

impl ASTNodeNumber {
	/// Creates a new numeric AST node.
	///
	/// # Parameters
	///
	/// - `number` - the numeric token to wrap
	///
	/// # Returns
	///
	/// A new [`ASTNodeNumber`] instance.
	pub fn new<T: Into<TokenNumber>>(number: T) -> ASTNodeNumber {
		ASTNodeNumber(number.into())
	}
}

impl ASTNodeRenderable for ASTNodeNumber {
	/// Returns the display name for this node.
	///
	/// The generated name has the form:
	/// `Number(<number>)`
	fn get_name(&self) -> ASTNodeName {
		let node_name: String = format!("Number({})", self.0.get_value());
		ASTNodeName::new(node_name)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ast::ASTNodeRenderable;
	use crate::token::types::TokenNumber;

	#[test]
	fn get_name_formats_numeric_literal() {
		let node = ASTNodeNumber::new(TokenNumber::new("1"));
		let name = node.get_name();

		assert_eq!(name.to_string(), "Number(1)");
	}
}
