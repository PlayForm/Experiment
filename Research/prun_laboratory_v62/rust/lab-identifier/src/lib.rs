//! Small reference identifier validator (no regex engine on authorization
//! path). Does NOT resolve paths or grant capabilities.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentifierKind {
	/// Human-readable but private *logical* label. Never use as a filesystem
	/// path.
	LogicalLabel,
	/// Base64url without padding; a 32-byte random identifier is 43 characters.
	OpaqueRoute,
}

#[derive(Debug, Clone, Copy)]
pub struct IdentifierSpec {
	pub kind:IdentifierKind,
	pub min_bytes:usize,
	pub max_bytes:usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
	InvalidSpec,
	InvalidLength,
	InvalidFirstByte,
	InvalidContinuationByte,
}

impl IdentifierSpec {
	pub fn validate(&self, value:&str) -> Result<(), Error> {
		if self.min_bytes == 0 || self.max_bytes < self.min_bytes || self.max_bytes > 4096 {
			return Err(Error::InvalidSpec);
		}
		let bytes = value.as_bytes();
		if bytes.len() < self.min_bytes || bytes.len() > self.max_bytes {
			return Err(Error::InvalidLength);
		}
		let Some((&first, rest)) = bytes.split_first() else {
			return Err(Error::InvalidLength);
		};
		match self.kind {
			IdentifierKind::LogicalLabel => {
				if !first.is_ascii_alphanumeric() {
					return Err(Error::InvalidFirstByte);
				}
				if !rest.iter().all(|b| b.is_ascii_alphanumeric() || b"_.:/-".contains(b)) {
					return Err(Error::InvalidContinuationByte);
				}
			},
			IdentifierKind::OpaqueRoute => {
				if !is_base64url(first) || !rest.iter().copied().all(is_base64url) {
					return Err(Error::InvalidContinuationByte);
				}
			},
		}
		Ok(())
	}

	/// ECMA-262-friendly JSON Schema pattern. Absolute-end lookahead avoids
	/// `$` accepting a final newline in some engines. Test across engines.
	pub fn json_schema_pattern(&self) -> Result<String, Error> {
		if self.min_bytes == 0 || self.max_bytes < self.min_bytes || self.max_bytes > 4096 {
			return Err(Error::InvalidSpec);
		}
		let min_rest = self.min_bytes - 1;
		let max_rest = self.max_bytes - 1;
		let head = match self.kind {
			IdentifierKind::LogicalLabel => "[A-Za-z0-9]",
			IdentifierKind::OpaqueRoute => "[A-Za-z0-9_-]",
		};
		let cont = match self.kind {
			IdentifierKind::LogicalLabel => "[A-Za-z0-9_.:/-]",
			IdentifierKind::OpaqueRoute => "[A-Za-z0-9_-]",
		};
		Ok(format!("^(?:{}{cont}{{{min_rest},{max_rest}}})(?![\\s\\S])", head))
	}
}

#[inline]
fn is_base64url(byte:u8) -> bool { byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_' }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn logical_accepts_literals_but_never_as_paths() {
		let spec = IdentifierSpec { kind:IdentifierKind::LogicalLabel, min_bytes:1, max_bytes:128 };
		for text in ["A", "a-b/c", "A/../"] {
			assert_eq!(spec.validate(text), Ok(()));
		}
		for text in ["", "\n", "A\n", "/abc", "Å", "a\0b"] {
			assert!(spec.validate(text).is_err());
		}
	}

	#[test]
	fn opaque_routes_are_exact_length_and_ascii() {
		let spec = IdentifierSpec { kind:IdentifierKind::OpaqueRoute, min_bytes:43, max_bytes:43 };
		assert_eq!(spec.validate(&"A".repeat(43)), Ok(()));
		assert!(spec.validate(&"A".repeat(42)).is_err());
		assert!(spec.validate(&("A".repeat(43) + "\n")).is_err());
	}

	#[test]
	fn generated_patterns_use_absolute_end_test() {
		let spec = IdentifierSpec { kind:IdentifierKind::LogicalLabel, min_bytes:1, max_bytes:128 };
		let generated = spec.json_schema_pattern().unwrap();
		assert!(generated.starts_with("^(?:"));
		assert!(generated.ends_with(r"(?![\s\S])"));
	}
}
