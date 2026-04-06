use crate::token::types::TokenOperatorType;
use crate::token::{TokenRenderable, TokenString};

#[derive(Debug)]
pub struct TokenOperator {
	operator_type: TokenOperatorType,
}

impl TokenOperator {
	pub fn new(operator_type: TokenOperatorType) -> TokenOperator {
		TokenOperator { operator_type }
	}

	fn get(&self) -> &TokenOperatorType {
		match self.operator_type {
			TokenOperatorType::Add => &TokenOperatorType::Add,
		}
	}
}

impl TokenRenderable for TokenOperator {
	fn as_token_string(&self) -> TokenString {
		TokenString::new(format!("Operator({})", self.get()))
	}
}
