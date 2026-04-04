use super::TokenString;

pub trait TokenRenderable {
	fn as_token_string(&self) -> TokenString;
}