use super::{TokenRenderable, TokenString};
use crate::token::types::{TokenNumber, TokenOperator};

#[derive(Debug, Clone)]
pub enum Token {
	Number(TokenNumber),
	Operator(TokenOperator),
}

impl TokenRenderable for Token {
	fn as_token_string(&self) -> TokenString {
		match self {
			Token::Number(number) => number.as_token_string(),
			Token::Operator(operator) => operator.as_token_string(),
		}
	}
}
