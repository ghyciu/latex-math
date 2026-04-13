use crate::token::types::TokenOperatorType;
use crate::token::{TokenRenderable, TokenName, TokenValue};

#[derive(Debug, Clone)]
/// An operator token.
pub struct TokenOperator {
	operator_type: TokenOperatorType,
}

impl TokenOperator {
	/// Creates a new operator token.
	pub fn new(operator_type: TokenOperatorType) -> TokenOperator {
		TokenOperator { operator_type }
	}

	fn get_type(&self) -> &TokenOperatorType {
		match self.operator_type {
			TokenOperatorType::Add => &TokenOperatorType::Add,
		}
	}
}

impl TokenRenderable for TokenOperator {
	fn get_value(&self) -> TokenValue {
		TokenValue::new(self.get_type().to_string())
	}

	fn get_name(&self) -> TokenName {
		TokenName::new(format!("Operator({})", self.get_value().to_string()))
	}
}
