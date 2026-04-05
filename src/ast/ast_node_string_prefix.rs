use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct ASTNodeStringPrefix(Vec<bool>);

impl ASTNodeStringPrefix {
	pub fn new() -> ASTNodeStringPrefix {
		ASTNodeStringPrefix(Vec::new())
	}

	pub fn child(&self, is_last: bool) -> ASTNodeStringPrefix {
		let mut levels = self.0.clone();
		levels.push(is_last);
		ASTNodeStringPrefix(levels)
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl Display for ASTNodeStringPrefix {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.0.is_empty() {
			return Ok(());
		}

		for level in &self.0[..self.0.len() - 1] {
			if *level {
				write!(f, "    ")?;
			} else {
				write!(f, "│   ")?;
			}
		}

		if self.0.last().unwrap() == &true {
			write!(f, "└── ")?;
		} else {
			write!(f, "├── ")?;
		}
		Ok(())
	}
}