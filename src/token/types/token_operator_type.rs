use std::fmt::Display;

#[derive(Debug, Clone)]
/// Kinds of operator tokens recognized by the tokenizer.
pub enum TokenOperatorType {
	/// Addition operator.
	Add,
}

impl Display for TokenOperatorType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TokenOperatorType::Add => write!(f, "+"),
		}
	}
}
