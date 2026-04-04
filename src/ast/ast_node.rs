use super::ast_node_types::{ASTNodeNumber, ASTNodeBinary};

#[derive(Debug)]
pub enum ASTNode {
	Number(ASTNodeNumber),
	Binary(ASTNodeBinary)
}