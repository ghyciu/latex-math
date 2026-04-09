use std::fmt::Display;

pub struct TokenValue(String);

impl TokenValue {
	pub fn new<T: Into<String>>(value: T) -> TokenValue {
		TokenValue(value.into())
	}
}

impl Display for TokenValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}