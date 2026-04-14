use crate::token::{TokenName, TokenRenderable, TokenValue};

/// A numeric token.
#[derive(Debug, Clone)]
pub struct TokenNumber(String);

impl TokenNumber {
	/// Creates a new numeric token.
	pub fn new<T: Into<String>>(number: T) -> TokenNumber {
		TokenNumber(number.into())
	}

	pub fn get(&self) -> &String {
		&self.0
	}
}

impl From<&str> for TokenNumber {
	fn from(number: &str) -> TokenNumber {
		TokenNumber(number.to_string())
	}
}

impl TokenRenderable for TokenNumber {
	fn get_value(&self) -> TokenValue {
		TokenValue::new(self.get().clone())
	}

	fn get_name(&self) -> TokenName {
		TokenName::new(format!("Number({})", self.get_value().to_string()))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_stores_number_string() {
		let token: TokenNumber = TokenNumber::new("1");
		assert_eq!(token.get(), "1");
	}

	#[test]
	fn from_str_creates_equivalent_token() {
		let token: TokenNumber = TokenNumber::from("1");
		assert_eq!(token.get(), "1");
	}

	#[test]
	fn renderable_value_matches_number() {
		let token: TokenNumber = TokenNumber::new("1");
		assert_eq!(token.get_value().to_string(), "1");
	}

	#[test]
	fn renderable_name_wraps_number_value() {
		let token: TokenNumber = TokenNumber::new("1");
		assert_eq!(token.get_name().to_string(), "Number(1)");
	}
}
