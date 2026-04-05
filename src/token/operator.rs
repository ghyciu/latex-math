use super::{OperatorType, TokenRenderable, TokenString};

#[derive(Debug, Clone)]
pub struct Operator {
	operator_type: OperatorType,
}

impl Operator {
	pub fn new(operator_type: OperatorType) -> Operator {
		Operator { operator_type }
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