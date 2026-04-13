use crate::ast::{ASTNode, ASTNodeRenderable, ASTNodeStringPrefix};
use std::fmt::Display;

/// A formatted string representation of an AST subtree.
///
/// The output is rendered in a tree-like layout using prefixes and branches.
#[derive(Clone)]
pub struct ASTNodeString<'a> {
	prefix: ASTNodeStringPrefix,
	node: &'a ASTNode,
}

impl<'a> ASTNodeString<'a> {
	/// Creates a new AST string renderer for a node.
	pub fn new(prefix: ASTNodeStringPrefix, node: &'a ASTNode) -> ASTNodeString<'a> {
		ASTNodeString { prefix, node }
	}
}

impl<'a> Display for ASTNodeString<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self.node {
			ASTNode::Number(number) => {
				write!(f, "{}{}\n", self.prefix, number.get_name())?;
			},
			ASTNode::Binary(binary) => {
				let left_string: ASTNodeString =
					ASTNodeString::new(self.prefix.child(false), binary.get_left());
				let right_string: ASTNodeString =
					ASTNodeString::new(self.prefix.child(true), binary.get_right());
				write!(
					f,
					"{}{}\n{}{}",
					self.prefix,
					binary.get_name(),
					left_string,
					right_string
				)?;
			},
			ASTNode::Unary(unary) => {
				let child_string: ASTNodeString = ASTNodeString::new(self.prefix.child(true), unary.get_child());
				write!(f, "{}{}\n{}", self.prefix, unary.get_name(), child_string)?;
			}
		}
		Ok({})
	}
}
