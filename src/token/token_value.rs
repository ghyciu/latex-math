use std::fmt::Display;

/// The printable value of a token.
pub struct TokenValue(String);

impl TokenValue {
	/// Creates a new token value wrapper.
	pub fn new<T: Into<String>>(value: T) -> TokenValue {
		TokenValue(value.into())
	}
}

impl Display for TokenValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_displays_input_string() {
		let value: TokenValue = TokenValue::new("1");
		assert_eq!(value.to_string(), "1");
	}
}
