use super::{TokenRenderable, TokenString};

#[derive(Debug)]
#[derive(Clone)]
pub struct Number(String);

impl Number {
	pub fn new(number: String) -> Number {
		Number(number)
	}

	pub fn get(&self) -> &String {
		&self.0
	}
}

impl TokenRenderable for Number {
	fn as_token_string(&self) -> TokenString {
		TokenString::new(format!("Number({})", self.get()))
	}
}