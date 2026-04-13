//! Abstract syntax tree support for mathematical expressions.
//!
//! This module defines the shared AST types and utilities used to represent
//! parsed expressions, render node names, and build AST structures.
//!
//! ## Re-exported Types
//!
//! - [`ASTNode`] — general AST node wrapper
//! - [`ASTNodeRenderable`] — trait for rendering node names
//! - [`ASTNodeName`] — human-readable node name representation
//! - [`ASTNodeString`] — string form of AST nodes
//! - [`ASTNodeStringPrefix`] — prefix used when formatting AST output
//! - [`ASTParser`] — parser for building AST structures
//! - [`ASTString`] — formatted AST output representation
//!
//! ## Concrete Node Types
//!
//! See [`node_types`] for built-in node implementations such as binary,
//! unary, and numeric literal nodes.

mod ast_node;
mod ast_node_renderable;
mod ast_node_string;
mod ast_node_string_prefix;
mod ast_parser;
mod ast_string;
mod ast_node_name;
pub mod node_types;

pub use ast_node::ASTNode;
pub use ast_node_string_prefix::ASTNodeStringPrefix;
pub use ast_parser::ASTParser;

pub use ast_node_renderable::ASTNodeRenderable;
pub use ast_node_string::ASTNodeString;
pub use ast_node_name::ASTNodeName;
pub use ast_string::ASTString;
