use super::{TokenName, TokenRenderable, TokenValue};
use crate::token::types::{TokenNumber, TokenOperator};

/// A lexical token recognized by the parser.
///
/// Tokens are currently either numeric literals or operators.
#[derive(Debug, Clone)]
pub enum Token {
	/// A numeric token.
	Number(TokenNumber),

	/// An operator token.
	Operator(TokenOperator),
}

impl From<TokenNumber> for Token {
	fn from(number: TokenNumber) -> Self {
		Token::Number(number)
	}
}

impl From<TokenOperator> for Token {
	fn from(operator: TokenOperator) -> Self {
		Token::Operator(operator)
	}
}

impl TokenRenderable for Token {
	/// Returns the token's value.
	fn get_value(&self) -> TokenValue {
		match self {
			Token::Number(number) => number.get_value(),
			Token::Operator(operator) => operator.get_value(),
		}
	}

	/// Returns the token's name.
	fn get_name(&self) -> TokenName {
		match self {
			Token::Number(number) => number.get_name(),
			Token::Operator(operator) => operator.get_name(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::token::types::{TokenNumber, TokenOperator, TokenOperatorType};

	#[test]
	fn number_token_converts_from_token_number() {
		let token: Token = TokenNumber::new("1").into();

		assert_eq!(token.get_value().to_string(), "1");
		assert_eq!(token.get_name().to_string(), "Number(1)");
	}

	#[test]
	fn operator_token_converts_from_token_operator() {
		let token: Token = TokenOperator::new(TokenOperatorType::Add).into();

		assert_eq!(token.get_value().to_string(), "+");
		assert_eq!(token.get_name().to_string(), "Operator(+)");
	}
}
