use crate::token::TokenName;
use std::fmt::{Display, Formatter, Result};

pub struct TokenNameList(Vec<TokenName>);

impl TokenNameList {
	pub fn new() -> TokenNameList {
		TokenNameList(Vec::new())
	}

	pub fn push(&mut self, token_string: TokenName) {
		self.0.push(token_string);
	}
}

impl Display for TokenNameList {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		for token_string in self.0.iter() {
			write!(f, "{}\n", token_string)?;
		}
		Ok({})
	}
}
