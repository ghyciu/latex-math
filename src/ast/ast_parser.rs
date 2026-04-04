use crate::ast::ast_node_types::{ASTNodeBinary, ASTNodeNumber};
use crate::ast::ASTNode;
use crate::token::Token;

pub struct ASTParser {
	tokens: Vec<Token>,
	pos: usize
}

impl ASTParser {
	pub fn new(tokens: Vec<Token>) -> ASTParser {
		ASTParser {
			tokens,
			pos: 0
		}
	}

	fn get(&self) -> Option<&Token> {
		self.tokens.get(self.pos)
	}

	fn next(&mut self) -> Option<Token> {
		let token = self.tokens.get(self.pos).cloned();
		self.pos += 1;
		token
	}

	pub fn parse(&mut self) -> ASTNode {
		self.parse_expression()
	}

	fn parse_expression(&mut self) -> ASTNode {
		let mut ast_node: ASTNode = self.parse_factor();
		while let Some(token) = self.get() {
			match token {
				Token::Operator(operator) => {
					let operator = self.next().unwrap();
					let right = self.parse_factor();
					ast_node = ASTNode::Binary(ASTNodeBinary::new(Box::new(ast_node), operator, Box::new(right)))
				},
				_ => break
			}
		}
		ast_node
	}

	fn parse_factor(&mut self) -> ASTNode {
		match self.next() {
			Some(Token::Number(number)) => ASTNode::Number(ASTNodeNumber::new(number)),
			_ => panic!()
		}
	}
}