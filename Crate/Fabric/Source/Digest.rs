//! Shape-only digest reference used across every domain.
//!
//! A digest is a commitment to exact bytes inside a controlled namespace;
//! exposing it can reveal content equality (`docs/03:46-48`). This value
//! validates lowercase-hex shape and emits an opaque reference. It never
//! hashes, encrypts, verifies a signature or looks up a byte store
//! (`docs/08:26-38`, `VALIDATION_REPORT.md:16-18`).

/// A 64-character lowercase-hex digest reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Digest {
    /// The supplied text, retained verbatim for shape inspection.
    pub text: String,
}

/// Text length of a SHA-256 lowercase-hex identity.
pub const DIGEST_TEXT_LENGTH: usize = 64;

impl Digest {
    /// Wrap supplied text without claiming it was computed.
    #[must_use]
    pub fn of(text: &str) -> Self {
        Self {
            text: text.to_owned(),
        }
    }

    /// Whether the text has the required digest shape.
    #[must_use]
    pub fn well_formed(&self) -> crate::verdict::Verdict {
        crate::verdict::Verdict::digest(&self.text)
    }

    /// An opaque content reference; equality here means text equality only.
    #[must_use]
    pub fn reference(&self) -> String {
        format!("sha256:{}", self.text)
    }

    /// Whether this reference alone can authenticate a recipient. It cannot
    /// (`docs/08:26-31`); recipient binding lives in the sealed grant.
    #[must_use]
    pub const fn authenticates_recipient() -> bool {
        false
    }
}
