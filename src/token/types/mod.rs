//! Concrete token types used by the tokenizer and parser.
//!
//! This module contains the token variants that represent parsed input
//! elements such as numbers and operators.
//!
//! ## Re-exported Types
//!
//! - [`TokenNumber`] — numeric literal token
//! - [`TokenOperator`] — operator token
//! - [`TokenOperatorType`] — classification of operator tokens

mod token_number;
mod token_operator;
mod token_operator_type;

pub use token_number::TokenNumber;
pub use token_operator::TokenOperator;
pub use token_operator_type::TokenOperatorType;
