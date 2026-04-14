use crate::ast::ASTNode;
use crate::ast::node_types::{ASTNodeBinary, ASTNodeNumber, ASTNodeUnary};
use crate::token::Token;

/// Parses a token stream into an AST.
///
/// The parser consumes tokens sequentially and builds AST nodes for
/// numbers, unary operators, and binary operators.
pub struct ASTParser<'a> {
	tokens: &'a Vec<Token>,
	pos: usize,
}

impl<'a> ASTParser<'a> {
	/// Creates a new AST parser for the provided token stream.
	pub fn new(tokens: &'a Vec<Token>) -> ASTParser<'a> {
		ASTParser { tokens, pos: 0 }
	}

	fn get(&self) -> Option<&Token> {
		self.tokens.get(self.pos)
	}

	fn next(&mut self) -> Option<Token> {
		let token = self.tokens.get(self.pos).cloned();
		self.pos += 1;
		token
	}

	/// Parses the full token stream into a single AST node.
	pub fn parse(&mut self) -> ASTNode {
		self.parse_expression()
	}

	fn parse_expression(&mut self) -> ASTNode {
		let mut ast_node: ASTNode = self.parse_factor();
		while let Some(token) = self.get() {
			match token {
				Token::Operator(_) => {
					let operator = self.next().unwrap();
					let right = self.parse_factor();
					ast_node = ASTNode::Binary(ASTNodeBinary::new(ast_node, operator, right))
				}
				_ => break,
			}
		}
		ast_node
	}

	fn parse_factor(&mut self) -> ASTNode {
		match self.next() {
			Some(Token::Number(number)) => ASTNode::Number(ASTNodeNumber::new(number)),
			Some(Token::Operator(_operator)) => {
				let operand = self.parse_factor();
				ASTNode::Unary(ASTNodeUnary::new(
					Token::Operator(_operator),
					operand,
				))
			}
			_ => panic!(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::token::types::{TokenNumber, TokenOperator, TokenOperatorType};

	fn token_from_number(value: &str) -> Token {
		Token::Number(TokenNumber::new(value))
	}

	fn token_from_operator(value: TokenOperatorType) -> Token {
		Token::Operator(TokenOperator::new(value))
	}

	#[test]
	fn parse_number_token_into_ast_node() {
		let number_token: Token = token_from_number("1");

		let tokens: Vec<Token> = vec![number_token];
		let root: ASTNode = ASTParser::new(&tokens).parse();

		assert_eq!(root.to_ast_node_string().to_string(), "Number(1)\n");
	}

	#[test]
	fn parse_unary_operator_token_stream() {
		let operator_token: Token = token_from_operator(TokenOperatorType::Add);
		let number_token: Token = token_from_number("1");

		// +1
		let tokens: Vec<Token> = vec![operator_token, number_token];
		let root: ASTNode = ASTParser::new(&tokens).parse();

		const EXPECTED_AST: &str = "\
		UnaryOperator(+)\n\
		└── Number(1)\n\
		";
		assert_eq!(root.to_ast_node_string().to_string(), EXPECTED_AST);
	}

	#[test]
	fn parse_binary_operator_token_stream() {
		let operator_token: Token = token_from_operator(TokenOperatorType::Add);
		let number_token_a: Token = token_from_number("1");
		let number_token_b: Token = token_from_number("2");

		// 1 + 2
		let tokens: Vec<Token> = vec![number_token_a, operator_token, number_token_b];
		let root: ASTNode = ASTParser::new(&tokens).parse();

		const EXPECTED_AST: &str = "\
		BinaryOperator(+)\n\
		├── Number(1)\n\
		└── Number(2)\n\
		";
		assert_eq!(root.to_ast_node_string().to_string(), EXPECTED_AST);
	}

	#[test]
	fn parse_operator_chain_is_left_associative() {
		let operator_token_a: Token = token_from_operator(TokenOperatorType::Add);
		let operator_token_b: Token = token_from_operator(TokenOperatorType::Add);
		let number_token_a: Token = token_from_number("1");
		let number_token_b: Token = token_from_number("2");
		let number_token_c: Token = token_from_number("3");

		// 1 + 2 + 3
		let tokens: Vec<Token> = vec![number_token_a, operator_token_a, number_token_b, operator_token_b, number_token_c];
		let root: ASTNode = ASTParser::new(&tokens).parse();

		const EXPECTED_AST: &str = "\
			BinaryOperator(+)\n\
			├── BinaryOperator(+)\n\
			│   ├── Number(1)\n\
			│   └── Number(2)\n\
			└── Number(3)\n\
			";
		assert_eq!(root.to_ast_node_string().to_string(), EXPECTED_AST);
	}
}