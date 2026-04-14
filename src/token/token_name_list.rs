use crate::token::{Token, TokenName, TokenRenderable};
use std::fmt::{Display, Formatter, Result};

/// A formatted list of token names.
pub struct TokenNameList {
	token_names: Vec<TokenName>,
}

impl TokenNameList {
	/// Creates a token name list from a token stream.
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::token::types::{TokenNumber, TokenOperator, TokenOperatorType};

	#[test]
	fn token_list_renders_each_name_on_its_own_line() {
		let token_number: Token = TokenNumber::new("1").into();
		let token_operator: Token = TokenOperator::new(TokenOperatorType::Add).into();

		let tokens: Vec<Token> = vec![token_number, token_operator];
		let token_names: TokenNameList = TokenNameList::new(&tokens);
		const EXPECTED_TOKEN_NAME_LIST: &str = "\
			Number(1)\n\
			Operator(+)\n\
			";

		assert_eq!(token_names.to_string(), EXPECTED_TOKEN_NAME_LIST);
	}
}
