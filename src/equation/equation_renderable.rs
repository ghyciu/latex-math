use crate::token::TokenStringList;

pub trait EquationRenderable {
	fn as_token_string_list(&self) -> TokenStringList;
}