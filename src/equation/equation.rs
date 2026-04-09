use super::EquationRenderable;
use crate::ast::ASTString;
use crate::token::{Token, TokenNameList, TokenParser, TokenRenderable};

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
	fn to_token_name_list(&self) -> TokenNameList {
		let mut token_name_list: TokenNameList = TokenNameList::new();
		for token in self.tokens.iter() {
			token_name_list.push(token.get_name());
		}
		token_name_list
	}

	fn to_ast_string(&self) -> ASTString {
		ASTString::new(&self.tokens)
	}
}
