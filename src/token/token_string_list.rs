use std::fmt::{Display, Formatter, Result};
use crate::token::TokenString;

pub struct TokenStringList(Vec<TokenString>);

impl TokenStringList {
	pub fn new() -> TokenStringList {
		TokenStringList(Vec::new())
	}

	pub fn push(&mut self, token_string: TokenString) {
		self.0.push(token_string);
	}
}

impl Display for TokenStringList {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		for token_string in self.0.iter() {
			write!(f, "{}\n", token_string)?;
		}
		Ok({})
	}
}