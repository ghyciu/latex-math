use crate::token::{Token, TokenName, TokenRenderable};
use std::fmt::{Display, Formatter, Result};

pub struct TokenNameList {
	token_names: Vec<TokenName>
}

impl TokenNameList {
	pub fn new(tokens: &Vec<Token>) -> TokenNameList {
		let token_names: Vec<TokenName> = tokens.iter().map(|token| token.get_name()).collect();
		TokenNameList { token_names }
	}
}

impl Display for TokenNameList {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		for token_string in self.token_names.iter() {
			write!(f, "{}\n", token_string)?;
		}
		Ok({})
	}
}
