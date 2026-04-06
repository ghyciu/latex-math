use std::fmt::Display;

pub struct TokenString(String);

impl TokenString {
	pub fn new(string: String) -> TokenString {
		TokenString(string)
	}
}

impl Display for TokenString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}
