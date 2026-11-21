//! Content-addressed identity for contract documents.
//!
//! Every artifact in `MANIFEST.sha256:1-295` is bound to a lowercase 64-hex
//! SHA-256 digest, and every schema field that carries a digest uses the
//! `^[a-f0-9]{64}$` pattern (`schemas/sample.schema.json:58-72`,
//! `schemas/v61-custody-request.schema.json:16-19`). A digest identifies bytes,
//! not a grant: knowledge of a hash never authenticates a recipient
//! (`docs/25-failure-analysis.md` F12/F13). This type validates shape only and
//! degrades with a warning instead of panicking. The manifest itself is read as
//! an identity scheme; it is deliberately not copied into this crate.

use crate::Warning;

/// A validated lowercase SHA-256 hex digest.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Digest {
    value: String,
}

impl Digest {
    /// Length of a bare SHA-256 hex digest.
    pub const HEX: usize = 64;
    /// Prefix of a content-addressed store reference.
    pub const CAS_PREFIX: &'static str = "cas://sha256/";

    /// Validate a bare 64-hex digest, returning a warning on malformed input.
    #[must_use]
    pub fn parse(value: &str) -> Result<Self, Warning> {
        let shaped = value.len() == Self::HEX
            && value.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase());
        match shaped {
            true => Ok(Self { value: value.to_owned() }),
            false => Err(Warning::InvalidValue {
                field: "sha256",
                detail: format!("expected 64 lowercase hex characters, found {value}"),
            }),
        }
    }

    /// Validate a `cas://sha256/<64hex>` reference.
    #[must_use]
    pub fn parse_cas(value: &str) -> Result<Self, Warning> {
        match value.strip_prefix(Self::CAS_PREFIX) {
            Some(body) => Self::parse(body).map(|_| Self { value: body.to_owned() }),
            None => Err(Warning::InvalidValue {
                field: "cas reference",
                detail: format!("expected a cas://sha256/ prefix, found {value}"),
            }),
        }
    }

    /// Borrow the validated digest text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}
