use crate::token::Token;
use crate::token::types::{TokenNumber, TokenOperator, TokenOperatorType};

/// Converts a character stream into tokens.
///
/// The parser recognizes numeric literals and operator characters.
pub struct TokenParser;

impl TokenParser {
	/// Creates a new token parser.
	pub fn new() -> TokenParser {
		TokenParser
	}

	/// Tokenizes a character stream into a list of tokens.
	pub fn tokenize(&self, chars: Vec<char>) -> Vec<Token> {
		let mut tokens: Vec<Token> = Vec::new();
		let mut i: usize = 0;

		while i < chars.len() {
			let char: char = chars[i];
			match char {
				// Whitespace
				char if char.is_whitespace() => {
					i += 1;
				}

				// Number
				char if char.is_ascii_digit() => {
					let mut number_token: String = char.to_string();
					i += 1;
					while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
						number_token.push(chars[i]);
						i += 1;
					}
					tokens.push(Token::Number(TokenNumber::new(number_token)))
				}

				// Operator
				'+' => {
					tokens.push(Token::Operator(TokenOperator::new(TokenOperatorType::Add)));
					i += 1;
				}

				// Miscellaneous
				_ => {
					i += 1;
				}
			}
		}
		tokens
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::token::TokenRenderable;

	#[test]
	fn tokenize_empty_input_returns_no_tokens() {
		let parser: TokenParser = TokenParser::new();
		const INPUT_STRING: &str = "";

		assert!(parser.tokenize(INPUT_STRING.chars().collect()).is_empty());
	}

	#[test]
	fn tokenize_ignores_whitespace_and_unknown_characters() {
		let parser: TokenParser = TokenParser::new();
		const INPUT_STRING: &str = " a \t\n";

		let tokens: Vec<Token> = parser.tokenize(INPUT_STRING.chars().collect());

		assert!(tokens.is_empty());
	}

	#[test]
	fn tokenize_parses_number_and_add_operator() {
		let parser: TokenParser = TokenParser::new();
		const INPUT_STRING: &str = "1 + 2";

		let tokens: Vec<Token> = parser.tokenize(INPUT_STRING.chars().collect());

		assert_eq!(tokens.len(), 3);
		assert_eq!(tokens[0].get_name().to_string(), "Number(1)");
		assert_eq!(tokens[1].get_name().to_string(), "Operator(+)");
		assert_eq!(tokens[2].get_name().to_string(), "Number(2)");
	}

	#[test]
	fn tokenize_parses_decimal_number() {
		let parser: TokenParser = TokenParser::new();
		const INPUT_STRING: &str = "3.1415";

		let tokens: Vec<Token> = parser.tokenize(INPUT_STRING.chars().collect());

		assert_eq!(tokens.len(), 1);
		assert_eq!(tokens[0].get_value().to_string(), "3.1415");
		assert_eq!(tokens[0].get_name().to_string(), "Number(3.1415)");
	}
}
