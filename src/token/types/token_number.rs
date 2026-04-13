use crate::token::{TokenRenderable, TokenName, TokenValue};

/// A numeric token.
#[derive(Debug, Clone)]
pub struct TokenNumber(String);

impl TokenNumber {
	/// Creates a new numeric token.
	pub fn new(number: String) -> TokenNumber {
		TokenNumber(number)
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