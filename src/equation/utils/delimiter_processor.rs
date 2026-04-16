use crate::equation::error::EquationError;
use crate::equation::error::types::DelimiterMismatchError;
use crate::equation::error::types::EmptyEquationError;

pub struct DelimiterProcessor<'a> {
	input: &'a str,
}

impl<'a> DelimiterProcessor<'a> {
	pub fn new(input: &'a str) -> DelimiterProcessor<'a> {
		DelimiterProcessor { input }
	}

	pub fn process(self) -> Result<String, EquationError> {
		let mut string: String = self.input.trim().to_string();

		if string.is_empty() {
			return Err(EmptyEquationError.into())
		}

		loop {
			self.check_mismatch(&string)?;
			if let Some(stripped) = self.strip(&string) {
				string = stripped;
			} else {
				break;
			}
		}

		if string.trim().is_empty() {
			return Err(EmptyEquationError.into());
		}

		Ok(string)
	}

	fn strip(&self, string: &str) -> Option<String> {
		// \begin{math} ... \end{math}
		const BEGIN_MATH_MODE: &str = r"\begin{math}";
		const END_MATH_MODE: &str = r"\end{math}";
		if string.starts_with(BEGIN_MATH_MODE) && string.ends_with(END_MATH_MODE) {
			let new_string: String = string[12..string.len() - 10].trim().to_string();
			return Some(new_string);
		}

		// $$ ... $$
		else if string.starts_with("$$") && string.ends_with("$$") && string.len() >= 4 {
			let new_string: String = string[2..string.len() - 2].trim().to_string();
			return Some(new_string);
		}

		// \( ... \)
		else if string.starts_with(r"\(") && string.ends_with(r"\)") && string.len() >= 4 {
			let new_string: String = string[2..string.len() - 2].trim().to_string();
			return Some(new_string);
		}

		// \[ ... \]
		else if string.starts_with(r"\[") && string.ends_with(r"\]") && string.len() >= 4 {
			let new_string: String = string[2..string.len() - 2].trim().to_string();
			return Some(new_string);
		}

		// $ ... $
		else if string.starts_with('$') && string.ends_with('$') && string.len() >= 2 {
			let new_string: String = string[1..string.len() - 1].trim().to_string();
			return Some(new_string);
		}

		None
	}

	fn check_mismatch(&self, string: &str) -> Result<(), EquationError> {
		// --- \begin{math} / \end{math} ---
		let mut begin_math_mode_count = 0;
		for _part in string.match_indices(r"\begin{math}") {
			begin_math_mode_count += 1;
		}
		let mut end_math_mode_count = 0;
		for _part in string.match_indices(r"\end{math}") {
			end_math_mode_count += 1;
		}
		if begin_math_mode_count != end_math_mode_count {
			return Err(DelimiterMismatchError.into())
		}

		Ok(())
	}
}