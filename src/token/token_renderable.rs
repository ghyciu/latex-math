use super::{TokenName, TokenValue};

pub trait TokenRenderable {
	fn get_value(&self) -> TokenValue;
	fn get_name(&self) -> TokenName;
}
