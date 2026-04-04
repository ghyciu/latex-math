use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum OperatorType {
	Add
}

impl Display for OperatorType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			OperatorType::Add => write!(f, "+"),
		}
	}
}