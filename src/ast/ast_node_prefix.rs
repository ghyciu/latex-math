use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct ASTNodePrefix(Vec<bool>);

impl ASTNodePrefix {
	pub fn new() -> ASTNodePrefix {
		ASTNodePrefix(Vec::new())
	}

	pub fn child(&self, is_last: bool) -> ASTNodePrefix {
		let mut levels = self.0.clone();
		levels.push(is_last);
		ASTNodePrefix(levels)
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl Display for ASTNodePrefix {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			if self.0.is_empty() {
				return Ok(());
			}

			for level in &self.0[..self.0.len() - 1]  {
				if *level {
					write!(f, "    ")?;
				} else {
					write!(f, "│   ")?;
				}
			}
			Ok(())
	}
}