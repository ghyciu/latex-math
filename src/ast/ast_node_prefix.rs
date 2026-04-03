use std::fmt::{Display, Formatter, Result};

pub struct ASTNodePrefix(Vec<bool>);

impl ASTNodePrefix {
	pub fn new() -> ASTNodePrefix {
		ASTNodePrefix(Vec::new())
	}

	pub fn get_child(&self, is_last: bool) -> ASTNodePrefix {
		let mut levels: Vec<bool> = self.0.clone();
		levels.push(is_last);
		ASTNodePrefix(levels)
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl Display for ASTNodePrefix {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		for level in self.0.iter() {
			if *level {
				write!(f, "│ ")?;
			} else {
				write!(f, "  ")?;
			}
		}
		Ok({})
	}
}