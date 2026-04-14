//! # LaTeX Math
//!
//! Command-line interface (CLI) tool and library for parsing and representing
//! mathematical expressions
//!
//! ## Modules
//!
//! - [`ast`] — Abstract Syntax Tree (AST) representation of mathematical expressions
//! - [`equation`] — Equation parsing and representation
//! - [`token`] — Tokenization of mathematical expressions
//!
//! This crate is intended to be used both as a library and as the backend
//! for the command-line interface provided by the binary target.

pub mod ast;
pub mod equation;
pub mod token;
