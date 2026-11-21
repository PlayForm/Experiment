//! Validated content digests and image references.
//!
//! Every v5 contract binds artifacts by a lowercase 64-hex SHA-256 digest
//! (`research-case.v5.schema.json:53-56`, `paper.v5.schema.json:104-114`,
//! `sql/v5_migration.sql:12-19`) or a `sha256:`-prefixed builder image digest
//! (`repository-adapter.v5.schema.json:73-76`). A digest is data, never proof:
//! a correct hash chain proves attribution and integrity of bytes, not the
//! truth of a claim (`README.md:57-58`, `V4_TRUST_BRIDGE.md:57-63`). This type
//! validates shape only and degrades with a warning instead of panicking.

use crate::warning::Warning;

/// A validated lowercase SHA-256 hex digest, with an optional image prefix.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Digest {
    value: String,
}

impl Digest {
    /// Length of a bare SHA-256 hex digest.
    pub const HEX: usize = 64;
    /// Prefix of a content-addressed builder image reference.
    pub const IMAGE_PREFIX: &'static str = "sha256:";

    /// Validate a bare 64-hex digest, returning a warning on malformed input.
    #[must_use]
    pub fn parse(value: &str) -> Result<Self, Warning> {
        let shaped = value.len() == Self::HEX
            && value.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase());
        match shaped {
            true => Ok(Self {
                value: value.to_owned(),
            }),
            false => Err(Warning::InvalidValue {
                field: "sha256",
                detail: format!("expected 64 lowercase hex characters, found {value}"),
            }),
        }
    }

    /// Validate a `sha256:<64hex>` builder image digest.
    #[must_use]
    pub fn parse_image(value: &str) -> Result<Self, Warning> {
        match value.strip_prefix(Self::IMAGE_PREFIX) {
            Some(body) => {
                Self::parse(body)?;
                Ok(Self {
                    value: value.to_owned(),
                })
            }
            None => Err(Warning::InvalidValue {
                field: "image digest",
                detail: format!("expected a sha256: prefix, found {value}"),
            }),
        }
    }

    /// Borrow the validated digest text (without an image prefix).
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Whether this digest was supplied with an image prefix.
    #[must_use]
    pub fn is_image(&self) -> bool {
        self.value.starts_with(Self::IMAGE_PREFIX)
    }
}
