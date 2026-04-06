use crate::token::TokenString;
use std::fmt::{Display, Formatter, Result};

pub struct TokenListString(Vec<TokenString>);

impl TokenListString {
	pub fn new() -> TokenListString {
		TokenListString(Vec::new())
	}

	pub fn push(&mut self, token_string: TokenString) {
		self.0.push(token_string);
	}
}

impl Display for TokenListString {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		for token_string in self.0.iter() {
			write!(f, "{}\n", token_string)?;
		}
		Ok({})
	}
}
