use crate::token::Token;
use crate::token::types::{TokenNumber, TokenOperator, TokenOperatorType};

pub struct TokenParser;

impl TokenParser {
	pub fn new() -> TokenParser {
		TokenParser
	}

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