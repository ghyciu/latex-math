//! Equation parsing and result handling.
//!
//! This module provides the main equation type, the result of parsing or
//! validation, and rendering utilities for displaying equations and their
//! outcomes.
//!
//! ## Re-exported Types
//!
//! - [`Equation`] — parsed equation model
//! - [`EquationResult`] — result returned when creating or validating an equation
//! - [`EquationRenderable`] — trait for rendering equation output
//! - [`EquationResultString`] — string representation of an equation result
//!
//! ## Errors
//!
//! Error-related types are available under [`error`].

mod equation;
mod equation_renderable;
mod equation_result;
mod equation_result_string;
pub mod error;
mod utils;

pub use equation::Equation;
pub use equation_renderable::EquationRenderable;
pub use equation_result::EquationResult;
pub use equation_result_string::EquationResultString;
