//! Token definitions and parsing utilities.
//!
//! This module contains the token model used by the parser, along with helper
//! traits and token parsing support.
//!
//! ## Re-exported Types
//!
//! - [`Token`] — general token type
//! - [`TokenNameList`] — formatted list of token names
//! - [`TokenName`] — token name wrapper
//! - [`TokenValue`] — token value wrapper
//! - [`TokenParser`] — parser for converting text into tokens
//! - [`TokenRenderable`] — trait for rendering token output
//!
//! ## Token Variants
//!
//! See [`types`] for concrete token types.

mod token;
mod token_name;
mod token_name_list;
mod token_parser;
mod token_renderable;
mod token_value;
pub mod types;

pub use token::Token;
pub use token_name_list::TokenNameList;

pub use token_name::TokenName;
pub use token_renderable::TokenRenderable;
pub use token_value::TokenValue;

pub use token_parser::TokenParser;
