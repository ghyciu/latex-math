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
