use std::fmt::Display;

pub struct TokenName(String);

impl TokenName {
	pub fn new<T: Into<String>>(string: T) -> TokenName {
		TokenName(string.into())
	}
}

impl Display for TokenName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}
