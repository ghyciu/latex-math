use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum TokenOperatorType {
	Add,
}

impl Display for TokenOperatorType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TokenOperatorType::Add => write!(f, "+"),
		}
	}
}
