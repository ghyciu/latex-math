use super::{TokenRenderable, TokenName, TokenValue};
use crate::token::types::{TokenNumber, TokenOperator};

#[derive(Debug, Clone)]
pub enum Token {
	Number(TokenNumber),
	Operator(TokenOperator),
}

impl TokenRenderable for Token {
	fn get_value(&self) -> TokenValue {
		match self {
			Token::Number(number) => number.get_value(),
			Token::Operator(operator) => operator.get_value(),
		}
	}
	
	fn get_name(&self) -> TokenName {
		match self {
			Token::Number(number) => number.get_name(),
			Token::Operator(operator) => operator.get_name(),
		}
	}
}
