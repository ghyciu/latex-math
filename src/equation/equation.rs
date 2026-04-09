use super::EquationRenderable;
use crate::ast::ASTString;
use crate::token::{Token, TokenListString, TokenParser, TokenRenderable};

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
		let token_parser: TokenParser = TokenParser::new();
		token_parser.tokenize(equation.chars().collect())
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
