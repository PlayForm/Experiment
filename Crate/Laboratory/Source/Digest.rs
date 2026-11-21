//! Validated content digests and content-addressed references.
//!
//! Every v6.1 record binds bytes by a lowercase 64-hex SHA-256 digest or a
//! `cas://sha256/<digest>` reference (`docs/08-data-and-storage.md:26-38`,
//! `docs/23-interface-inventory.md:37-41`). A digest identifies bytes, not a
//! grant: knowledge of a hash never authenticates a recipient and never proves
//! a scientific claim (`docs/00-system-constitution.md:53-55`,
//! `docs/25-failure-analysis.md` F12/F13). This type validates shape only and
//! degrades with a warning instead of panicking.

use crate::warning::Warning;

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
            true => Ok(Self {
                value: value.to_owned(),
            }),
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
            Some(body) => {
                Self::parse(body)?;
                Ok(Self {
                    value: body.to_owned(),
                })
            }
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
