use super::{TokenRenderable, TokenString};

#[derive(Debug)]
pub struct Number(String);

impl Number {
	pub fn new(number: String) -> Number {
		Number(number)
	}

	fn get(&self) -> &String {
		&self.0
	}
}

impl TokenRenderable for Number {
	fn as_token_string(&self) -> TokenString {
		TokenString::new(format!("Number({})", self.get()))
	}
}