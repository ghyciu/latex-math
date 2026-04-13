use super::{EquationError, EquationRenderable, EquationResult};
use crate::ast::ASTString;
use crate::equation::errors::EmptyEquationError;
use crate::token::{Token, TokenNameList, TokenParser};

#[derive(Debug)]
pub struct Equation {
	tokens: Vec<Token>,
}

impl Equation {
	pub fn new(equation: &String) -> EquationResult {
		let equation: String = equation.trim().to_string();
		if equation.is_empty() {
			return Err(EquationError::EmptyEquationError(EmptyEquationError))
		}
		Ok(Equation {
			tokens: Self::tokenize(equation),
		})
	}

	fn tokenize(equation: String) -> Vec<Token> {
		let token_parser: TokenParser = TokenParser::new();
		token_parser.tokenize(equation.chars().collect())
	}
}

impl EquationRenderable for Equation {
	fn to_token_name_list(&self) -> TokenNameList {
		TokenNameList::new(&self.tokens)
	}

	fn to_ast_string(&self) -> ASTString {
		ASTString::new(&self.tokens)
	}
}
