//! Error types used by equation parsing and validation.
//!
//! This module defines the shared error infrastructure for equation-related
//! operations and exposes concrete error kinds through submodules.
//!
//! ## Re-exports
//!
//! - [`EquationError`] — enum covering all equation error variants
//! - [`EquationErrorRenderable`] — trait for converting errors into displayable output
//!
//! ## Submodules
//!
//! - [`types`] — concrete error types

mod equation_error;
mod equation_error_renderable;
pub mod types;

pub use equation_error::EquationError;
pub use equation_error_renderable::EquationErrorRenderable;
