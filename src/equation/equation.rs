use super::EquationRenderable;
use crate::ast::ASTString;
use crate::token::types::{TokenNumber, TokenOperator, TokenOperatorType};
use crate::token::{Token, TokenListString, TokenRenderable};

#[derive(Debug)]
pub struct Equation {
	tokens: Vec<Token>,
}

impl Equation {
	pub fn new(equation: String) -> Equation {
		Equation {
			tokens: Self::tokenize(equation),
		}
	}

	fn tokenize(equation: String) -> Vec<Token> {
		let mut tokens: Vec<Token> = Vec::new();
		let chars: Vec<char> = equation.chars().collect();
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

impl EquationRenderable for Equation {
	fn to_token_string_list(&self) -> TokenListString {
		let mut token_string_list: TokenListString = TokenListString::new();
		for token in self.tokens.iter() {
			token_string_list.push(token.as_token_string());
		}
		token_string_list
	}

	fn to_ast_string(&self) -> ASTString {
		ASTString::new(&self.tokens)
	}
}
