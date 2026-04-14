use std::fmt::Display;

/// Prefix information used when rendering an AST as a tree.
#[derive(Debug, Clone)]
pub struct ASTNodeStringPrefix(Vec<bool>);

impl ASTNodeStringPrefix {
	/// Creates an empty prefix.
	pub fn new() -> ASTNodeStringPrefix {
		ASTNodeStringPrefix(Vec::new())
	}

	/// Creates a child prefix.
	///
	/// The `is_last` flag determines whether the branch is the last child.
	pub fn child(&self, is_last: bool) -> ASTNodeStringPrefix {
		let mut levels = self.0.clone();
		levels.push(is_last);
		ASTNodeStringPrefix(levels)
	}

	/// Returns `true` if the prefix contains no levels.
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_creates_empty_prefix() {
		let prefix = ASTNodeStringPrefix::new();

		assert!(prefix.is_empty());
		assert_eq!(prefix.to_string(), "");
	}

	#[test]
	fn child_appends_last_branch_marker() {
		let prefix = ASTNodeStringPrefix::new().child(true);

		assert!(!prefix.is_empty());
		assert_eq!(prefix.to_string(), "└── ");
	}

	#[test]
	fn child_appends_non_last_branch_marker() {
		let prefix = ASTNodeStringPrefix::new().child(false);

		assert!(!prefix.is_empty());
		assert_eq!(prefix.to_string(), "├── ");
	}

	#[test]
	fn nested_prefix_renders_intermediate_vertical_line() {
		let prefix = ASTNodeStringPrefix::new().child(false).child(true);

		assert_eq!(prefix.to_string(), "│   └── ");
	}

	#[test]
	fn nested_prefix_renders_intermediate_spacing_for_last_branch() {
		let prefix = ASTNodeStringPrefix::new().child(true).child(false);

		assert_eq!(prefix.to_string(), "    ├── ");
	}
}
