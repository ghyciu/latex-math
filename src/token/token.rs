use super::{Number, Operator, TokenRenderable, TokenString};

#[derive(Debug, Clone)]
pub enum Token {
	Number(Number),
	Operator(Operator),
}

impl TokenRenderable for Token {
	fn as_token_string(&self) -> TokenString {
		match self {
			Token::Number(number) => number.as_token_string(),
			Token::Operator(operator) => operator.as_token_string(),
		}
	}
}
