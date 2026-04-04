use crate::ast::ast_node_string::ASTNodeString;
use crate::ast::ASTNodePrefix;

pub trait ASTNodeRenderable {
	fn as_ast_node_string(&self, prefix: ASTNodePrefix, is_last: bool) -> ASTNodeString;
}