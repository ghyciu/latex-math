//! Concrete equation error types.
//!
//! These types represent specific failure cases that can occur while
//! parsing or handling equations.
//!
//! ## Re-exported Types
//!
//! - [`EmptyEquationError`] — error returned when the input equation is empty

mod empty_equation_error;
mod delimiter_mismatch_error;

pub use empty_equation_error::EmptyEquationError;
pub use delimiter_mismatch_error::DelimiterMismatchError;