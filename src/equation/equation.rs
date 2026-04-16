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
		let mut equation: String = equation.into().trim().to_string();

		if equation.is_empty() {
			return EquationResult::err(EmptyEquationError);
		}

		equation = remove_math_mode_delimiters(equation);

		if equation.trim().is_empty() {
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

/// Removes supported LaTeX math-mode delimiters from the outside of an equation.
///
/// This keeps stripping matching wrappers until no more supported delimiters remain,
/// so nested forms like `$$ \( 1+2 \) $$` are normalized correctly.
fn remove_math_mode_delimiters(mut equation: String) -> String {
	loop {
		let trimmed = equation.trim();

		let mut changed = false;

		// \begin{math} ... \end{math}
		let begin = r"\begin{math}";
		let end = r"\end{math}";
		if trimmed.starts_with(begin) && trimmed.ends_with(end) {
			equation = trimmed[begin.len()..trimmed.len() - end.len()]
				.trim()
				.to_string();
			changed = true;
		}

		// $$ ... $$
		else if trimmed.starts_with("$$") && trimmed.ends_with("$$") && trimmed.len() >= 4 {
			equation = trimmed[2..trimmed.len() - 2].trim().to_string();
			changed = true;
		}

		// \( ... \)
		else if trimmed.starts_with(r"\(") && trimmed.ends_with(r"\)") && trimmed.len() >= 4 {
			equation = trimmed[2..trimmed.len() - 2].trim().to_string();
			changed = true;
		}

		// \[ ... \]
		else if trimmed.starts_with(r"\[") && trimmed.ends_with(r"\]") && trimmed.len() >= 4 {
			equation = trimmed[2..trimmed.len() - 2].trim().to_string();
			changed = true;
		}

		// $ ... $
		else if trimmed.starts_with('$') && trimmed.ends_with('$') && trimmed.len() >= 2 {
			equation = trimmed[1..trimmed.len() - 1].trim().to_string();
			changed = true;
		}

		if !changed {
			break;
		}
	}

	equation
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
	fn new_trims_whitespace_before_tokenizing() {
		let result: EquationResult = Equation::new("1+2");
		const EXPECTED_EQUATION: &str = "\x1b[48;5;34m\x1b[38;5;255m\x1b[1m EQUATION \x1b[0m 1+2";

		assert_eq!(result.to_string(), EXPECTED_EQUATION);
	}

	#[test]
	fn new_removes_inline_math_delimiters() {
		let result: EquationResult = Equation::new(r"\( 1+2 \)");
		const EXPECTED_EQUATION: &str = "\x1b[48;5;34m\x1b[38;5;255m\x1b[1m EQUATION \x1b[0m 1+2";

		assert_eq!(result.to_string(), EXPECTED_EQUATION);
	}

	#[test]
	fn new_removes_display_math_delimiters() {
		let result: EquationResult = Equation::new("$$ 1+2 $$");
		const EXPECTED_EQUATION: &str = "\x1b[48;5;34m\x1b[38;5;255m\x1b[1m EQUATION \x1b[0m 1+2";

		assert_eq!(result.to_string(), EXPECTED_EQUATION);
	}

	#[test]
	fn new_removes_nested_math_delimiters() {
		let result: EquationResult = Equation::new("$$ \\( 1+2 \\) $$");
		const EXPECTED_EQUATION: &str = "\x1b[48;5;34m\x1b[38;5;255m\x1b[1m EQUATION \x1b[0m 1+2";

		assert_eq!(result.to_string(), EXPECTED_EQUATION);
	}

	#[test]
	fn new_removes_begin_end_math_delimiters() {
		let result: EquationResult = Equation::new(r"\begin{math} 1+2 \end{math}");
		const EXPECTED_EQUATION: &str = "\x1b[48;5;34m\x1b[38;5;255m\x1b[1m EQUATION \x1b[0m 1+2";

		assert_eq!(result.to_string(), EXPECTED_EQUATION);
	}

	#[test]
	fn new_removes_bracket_math_delimiters() {
		let result: EquationResult = Equation::new(r"\[ 1+2 \]");
		const EXPECTED_EQUATION: &str = "\x1b[48;5;34m\x1b[38;5;255m\x1b[1m EQUATION \x1b[0m 1+2";

		assert_eq!(result.to_string(), EXPECTED_EQUATION);
	}

	#[test]
	fn new_returns_error_when_removing_delimiters_results_in_empty_input() {
		let result: EquationResult = Equation::new("$$$$");
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
