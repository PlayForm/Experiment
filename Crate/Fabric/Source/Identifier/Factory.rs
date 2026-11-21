//! Typed identifier constraints and the generated validator.
//!
//! From one private specification this factory describes both the byte-level
//! parser and its optional JSON Schema `pattern`. The generated pattern uses
//! an explicit end-of-input assertion so it does not inherit the
//! `(?![\\s\\S])` final-newline bug reproduced in `tests/validate.py:57-73`.
//! Worker-supplied patterns never change the authoritative parser
//! (`docs/01:7-15`, `docs/03:5-18`).

use crate::verdict::Verdict;
use super::noncanonical::Noncanonical;

/// Largest accepted logical label, matching the reference spec.
pub const MAX_BYTES: usize = 128;

/// Bounded, privately stored identifier constraints.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Factory {
    /// Smallest accepted byte count.
    pub min_bytes: usize,
    /// Largest accepted byte count.
    pub max_bytes: usize,
    /// Extra continuation bytes, fixed to `_.:/-` in this design.
    pub continuation_extra: &'static str,
    /// How noncanonical input is treated.
    pub noncanonical: Noncanonical,
}

impl Factory {
    /// The single authored spec used by the validating examples.
    #[must_use]
    pub const fn canonical() -> Self {
        Self {
            min_bytes: 1,
            max_bytes: MAX_BYTES,
            continuation_extra: "_.:/-",
            noncanonical: Noncanonical::Reject,
        }
    }

    /// Whether the spec itself is bounded and unambiguous.
    #[must_use]
    pub const fn spec_valid(&self) -> Verdict {
        match self.min_bytes >= 1 && self.max_bytes >= self.min_bytes && self.max_bytes <= MAX_BYTES {
            true => Verdict::Allow,
            false => Verdict::Deny,
        }
    }

    /// Validate raw bytes against the typed constraints. Returns the same
    /// result as the Python mirror for the cited conformance vectors
    /// (`tests/validate.py:99-104`).
    #[must_use]
    pub fn validate(&self, raw: &str) -> Verdict {
        let bytes = raw.as_bytes();
        self.spec_valid().and(Verdict::of(
            (self.min_bytes..=self.max_bytes).contains(&bytes.len())
                && bytes[0..1]
                    .first()
                    .is_some_and(|first| first.is_ascii_alphanumeric())
                && bytes[1..].iter().all(|byte| {
                    byte.is_ascii_alphanumeric() || self.continuation_extra.contains(*byte as char)
                }),
        ))
    }

    /// Generate the ECMA-262-friendly pattern with an absolute end assertion
    /// (`docs/03:20-33`, `rust/lab-identifier/src/lib.rs:58-75`).
    #[must_use]
    pub fn json_schema_pattern(&self) -> String {
        let rest_min = self.min_bytes.saturating_sub(1);
        let rest_max = self.max_bytes.saturating_sub(1);
        format!(
            "^(?:[A-Za-z0-9][A-Za-z0-9_.:/-]{{{rest_min},{rest_max}}})(?![\\s\\S])"
        )
    }
}
