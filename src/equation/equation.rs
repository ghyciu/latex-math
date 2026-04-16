use super::{EquationRenderable, EquationResult, EquationResultString};
use crate::ast::ASTString;
use crate::equation::error::types::EmptyEquationError;
use crate::equation::utils::DelimiterProcessor;
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
		let equation: String = equation.into();

		let equation: String = match DelimiterProcessor::new(&equation).process() {
			Ok(string) => string,
			Err(error) => return EquationResult::err(error),
		};

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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_returns_error_for_empty_input() {
		let result: EquationResult = Equation::new("");
		const EXPECTED_ERROR: &str = "\x1b[48;5;203m\x1b[38;5;255m\x1b[1m ERROR \
		\x1b[0m \x1b[40m\x1b[38;5;203mEmptyEquationError\x1b[0m: Equation cannot be empty";

		assert_eq!(result.to_string(), EXPECTED_ERROR);
	}

	#[test]
	fn to_token_name_list_formats_token_names() {
		let equation: Equation = Equation::new("1+2").unwrap();
		const EXPECTED_TOKEN_NAME_LIST: &str = "\
			Number(1)\n\
			Operator(+)\n\
			Number(2)\n\
			";

		assert_eq!(
			equation.to_token_name_list().to_string(),
			EXPECTED_TOKEN_NAME_LIST
		);
	}

	#[test]
	fn to_ast_string_produces_ast_output() {
		let equation: Equation = Equation::new(&"1+2".to_string()).unwrap();
		const EXPECTED_AST_STRING: &str = "\
			BinaryOperator(+)\n\
			├── Number(1)\n\
			└── Number(2)\n\
			";
		assert_eq!(equation.to_ast_string().to_string(), EXPECTED_AST_STRING);
	}
}
