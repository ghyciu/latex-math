use super::{ASTNode, ASTParser};
use crate::token::Token;
use std::fmt::Display;

/// A formatted AST representation built from a token stream.
pub struct ASTString(ASTNode);

impl ASTString {
	/// Parses tokens and stores the resulting AST root.
	pub fn new(tokens: &Vec<Token>) -> ASTString {
		let mut ast_parser: ASTParser = ASTParser::new(tokens);
		let root: ASTNode = ast_parser.parse();
		ASTString(root)
	}
}

impl Display for ASTString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0.to_ast_node_string())?;
		Ok({})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::token::types::{TokenNumber, TokenOperator, TokenOperatorType};

	fn token_from_number(value: &str) -> Token {
		Token::Number(TokenNumber::new(value))
	}

	fn token_from_operator(value: TokenOperatorType) -> Token {
		Token::Operator(TokenOperator::new(value))
	}

	#[test]
	fn formats_number_token_stream() {
		let number_token: Token = token_from_number("1");

		let tokens: Vec<Token> = vec![number_token];
		let string: String = ASTString::new(&tokens).to_string();

		assert_eq!(string, "Number(1)\n");
	}

	#[test]
	fn formats_unary_operator_token_stream() {
		let operator_token: Token = token_from_operator(TokenOperatorType::Add);
		let number_token: Token = token_from_number("1");

		let tokens: Vec<Token> = vec![operator_token, number_token];
		let string: String = ASTString::new(&tokens).to_string();

		const EXPECTED_STRING: &str = "\
		UnaryOperator(+)\n\
		└── Number(1)\n\
		";
		assert_eq!(string, EXPECTED_STRING);
	}

	#[test]
	fn formats_binary_operator_token_stream() {
		let operator_token: Token = token_from_operator(TokenOperatorType::Add);
		let number_token_a: Token = token_from_number("1");
		let number_token_b: Token = token_from_number("2");

		let tokens: Vec<Token> = vec![number_token_a, operator_token, number_token_b];
		let string: String = ASTString::new(&tokens).to_string();

		const EXPECTED_STRING: &str = "\
		BinaryOperator(+)\n\
		├── Number(1)\n\
		└── Number(2)\n\
		";
		assert_eq!(string, EXPECTED_STRING);
	}

	#[test]
	fn formats_left_associative_operator_chain() {
		let operator_token_a: Token = token_from_operator(TokenOperatorType::Add);
		let operator_token_b: Token = token_from_operator(TokenOperatorType::Add);
		let number_token_a: Token = token_from_number("1");
		let number_token_b: Token = token_from_number("2");
		let number_token_c: Token = token_from_number("3");

		let tokens: Vec<Token> = vec![
			number_token_a,
			operator_token_a,
			number_token_b,
			operator_token_b,
			number_token_c,
		];
		let string: String = ASTString::new(&tokens).to_string();

		const EXPECTED_STRING: &str = "\
			BinaryOperator(+)\n\
			├── BinaryOperator(+)\n\
			│   ├── Number(1)\n\
			│   └── Number(2)\n\
			└── Number(3)\n\
			";
		assert_eq!(string, EXPECTED_STRING);
	}

	#[test]
	#[should_panic]
	fn empty_token_stream_panics() {
		let tokens: Vec<Token> = vec![];
		let _string = ASTString::new(&tokens).to_string();
	}

	#[test]
	#[should_panic]
	fn trailing_operator_panics() {
		let tokens: Vec<Token> = vec![
			token_from_number("1"),
			token_from_operator(TokenOperatorType::Add),
		];
		let _string = ASTString::new(&tokens).to_string();
	}
}
