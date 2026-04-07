use crate::token::{TokenRenderable, TokenString};

#[derive(Debug, Clone)]
pub struct TokenNumber(String);

impl TokenNumber {
	pub fn new(number: String) -> TokenNumber {
		TokenNumber(number)
	}

	pub fn get(&self) -> &String {
		&self.0
	}
}

impl TokenRenderable for TokenNumber {
	fn as_token_string(&self) -> TokenString {
		TokenString::new(format!("Number({})", self.get()))
	}
}
