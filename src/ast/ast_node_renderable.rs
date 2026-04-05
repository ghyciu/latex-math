use crate::ast::ast_node_string::ASTNodeString;
use crate::ast::ASTNodeStringPrefix;

pub trait ASTNodeRenderable {
	fn to_ast_node_string(&self, prefix: ASTNodeStringPrefix) -> ASTNodeString;
}