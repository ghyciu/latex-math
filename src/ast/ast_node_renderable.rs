use crate::ast::ASTNodeName;

/// Trait for AST node types that can provide a display name.
///
/// This is primarily used for rendering and debugging AST structures.
pub trait ASTNodeRenderable {
	/// Returns the display name for the node.
	fn get_name(&self) -> ASTNodeName;
}
