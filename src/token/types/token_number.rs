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

impl TokenRenderable for TokenNumber {
	fn get_value(&self) -> TokenValue {
		TokenValue::new(self.get().clone())
	}

	fn get_name(&self) -> TokenName {
		TokenName::new(format!("Number({})", self.get_value().to_string()))
	}
}
