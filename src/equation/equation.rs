use super::{EquationRenderable, EquationResult, EquationResultString};
use crate::ast::ASTString;
use crate::equation::error::types::EmptyEquationError;
use crate::token::{Token, TokenNameList, TokenParser};

/// A parsed mathematical equation.
///
/// The equation stores its tokenized representation and can produce
/// formatted token, result, and AST output.
#[derive(Debug)]
pub struct Equation {
	tokens: Vec<Token>,
}

impl Equation {
	/// Creates a new equation from an input string.
	///
	/// Empty or whitespace-only input returns an error result.
	pub fn new<T: Into<String>>(equation: T) -> EquationResult {
		let equation: String = equation.into().trim().to_string();

		if equation.is_empty() {
			return EquationResult::err(EmptyEquationError);
		}

		EquationResult::ok(Equation {
			tokens: Equation::tokenize(equation),
		})
	}

	fn tokenize(equation: String) -> Vec<Token> {
		let token_parser: TokenParser = TokenParser::new();
		token_parser.tokenize(equation.chars().collect())
	}
}

impl EquationRenderable for Equation {
	/// Returns a formatted equation result string.
	fn to_result_string(&self) -> EquationResultString {
		EquationResultString::ok(&self.tokens)
	}

	/// Returns the equation as a list of token names.
	fn to_token_name_list(&self) -> TokenNameList {
		TokenNameList::new(&self.tokens)
	}

	/// Returns the equation as a formatted AST string.
	fn to_ast_string(&self) -> ASTString {
		ASTString::new(&self.tokens)
	}
}
