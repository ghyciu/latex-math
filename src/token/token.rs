use super::{Number, TokenString, TokenRenderable};

#[derive(Debug)]
pub enum Token {
	Number(Number)
}

impl TokenRenderable for Token {
	fn as_token_string(&self) -> TokenString {
		match self {
			Token::Number(number) => number.as_token_string()
		}
	}
}