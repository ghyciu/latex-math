use crate::ast::ASTNodeName;

pub trait ASTNodeRenderable {
	fn get_name(&self) -> ASTNodeName;
}
