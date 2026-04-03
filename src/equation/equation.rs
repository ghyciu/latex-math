use crate::token::{Number, Token, TokenRenderable};

#[derive(Debug)]
pub struct Equation {
	tokens: Vec<Token>
}

impl Equation {
	pub fn new(equation: String) -> Equation {
		Equation {
			tokens: Self::tokenize(equation)
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
					tokens.push(Token::Number(Number::new(number_token)))
				}

				// Miscellaneous
				_ => {
					i += 1;
				}
			}
		}

		return tokens;
	}

	pub fn print_tokens(&self) {
		for token in &self.tokens {
			println!("{}", token.as_token_string());
		}
	}
}