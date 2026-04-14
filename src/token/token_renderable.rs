use super::{TokenName, TokenValue};

/// Trait for token types that can expose a display name and value.
///
/// This is used when rendering token streams and equation output.
pub trait TokenRenderable {
	/// Returns the token's underlying value.
	fn get_value(&self) -> TokenValue;

	/// Returns a human-readable token name.
	fn get_name(&self) -> TokenName;
}
