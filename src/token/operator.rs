use super::{OperatorType, Token, TokenRenderable, TokenString};

#[derive(Debug)]
pub struct Operator {
	operator_type: OperatorType,
	left: Option<Box<Token>>,
	right: Option<Box<Token>>
}

impl Operator {
	pub fn new(operator_type: OperatorType) -> Operator {
		Operator {
			operator_type,
			left: None,
			right: None
		}
	}

	fn get(&self) -> &OperatorType {
		match self.operator_type {
			OperatorType::Add => &OperatorType::Add,
		}
	}
}

impl TokenRenderable for Operator {
	fn as_token_string(&self) -> TokenString {
		TokenString::new(format!("Operator({})", self.get()))
	}
}