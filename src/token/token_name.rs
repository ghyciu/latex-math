use std::fmt::Display;

/// A human-readable token name.
pub struct TokenName(String);

impl TokenName {
	/// Creates a new token name wrapper.
	pub fn new<T: Into<String>>(string: T) -> TokenName {
		TokenName(string.into())
	}
}

impl Display for TokenName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}
