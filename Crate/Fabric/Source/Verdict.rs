//! Structural allow/deny verdict shared by every typed decision in the crate.
//!
//! A decision is a value, never a branch at a call site. Conjuncts are folded
//! with [`Verdict::and`], mirroring the capability formula
//! `ValidIssuerSignature && CurrentEpoch && ... && FreshNonce`
//! (`docs/04:31-38`). No verdict ever asserts that cryptography ran.

/// One structural outcome of a typed check.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Verdict {
    /// The structural preconditions hold; nothing was executed or released.
    Allow,
    /// At least one precondition failed; the caller must fail closed.
    Deny,
}

impl Verdict {
    /// Conjoin two verdicts; a single denial fails the whole decision.
    #[must_use]
    pub const fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::Allow, Self::Allow) => Self::Allow,
            _ => Self::Deny,
        }
    }

    /// Lift a caller-supplied condition without a branch at the call site.
    #[must_use]
    pub const fn of(condition: bool) -> Self {
        match condition {
            true => Self::Allow,
            false => Self::Deny,
        }
    }

    /// Accept a non-empty supplied string.
    #[must_use]
    pub fn filled(value: &str) -> Self {
        Self::of(!value.is_empty())
    }

    /// Accept a 64-character lowercase-hex digest shape only; no hashing.
    /// Continuation bytes are restricted to lowercase hex explicitly
    /// (`tests/generate_contracts.py:11`, `schemas/*: HASH`).
    #[must_use]
    pub fn digest(value: &str) -> Self {
        Self::of(
            value.len() == crate::digest::DIGEST_TEXT_LENGTH
                && value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
        )
    }

    /// Accept a base64url-without-padding opaque identifier of exact length
    /// (`docs/03:9`, `tests/generate_contracts.py:10`).
    #[must_use]
    pub fn opaque(value: &str) -> Self {
        Self::of(
            value.len() == crate::OPAQUE_ID_LENGTH
                && value
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_'),
        )
    }

    /// The wire spelling for a trusted audit record.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Deny => "deny",
        }
    }

    /// Whether this verdict refuses the operation.
    #[must_use]
    pub const fn refuses(self) -> bool {
        matches!(self, Self::Deny)
    }

    /// The verdict after applying a named collision outcome: any collision is
    /// fatal and version conflicts are never waived (`docs/01:60-67`,
    /// `README.md:25-26`).
    #[must_use]
    pub const fn after(self, collision: crate::identifier::collision::Collision) -> Self {
        match collision {
            crate::identifier::collision::Collision::Clear => self,
            crate::identifier::collision::Collision::Namespace
            | crate::identifier::collision::Collision::Version
            | crate::identifier::collision::Collision::Semantic => Self::Deny,
        }
    }
}
