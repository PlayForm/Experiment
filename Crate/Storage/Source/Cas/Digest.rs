//! A 64-hex SHA-256 identity, validated by shape only.
//!
//! The digest is an identity, never an authorization: this type does not hash
//! bytes and does not authenticate a recipient
//! (`docs/08-data-and-storage.md:26-31`).

/// A content address carried by reference.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Digest {
    value: String,
}

impl Digest {
    /// Wrap a digest string without hashing anything.
    #[must_use]
    pub fn of(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }

    /// An unset digest used only before a chain is bound.
    #[must_use]
    pub fn zero() -> Self {
        Self::of("")
    }

    /// The wrapped string.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Whether the value is a 64-character hexadecimal string.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.value.len() == crate::DIGEST_LENGTH
            && self.value.bytes().all(|byte| byte.is_ascii_hexdigit())
    }

    /// The canonical CAS URI for these bytes.
    #[must_use]
    pub fn uri(&self) -> String {
        format!("{}{}", crate::CAS_SCHEME, self.value)
    }

    /// Whether two identities name the same bytes.
    #[must_use]
    pub fn matches(&self, other: &Self) -> crate::permission::Permission {
        match self == other {
            true => crate::permission::Permission::Allow,
            false => crate::permission::Permission::Deny,
        }
    }

    /// A safe description that never claims the digest is valid.
    #[must_use]
    pub fn describe(&self) -> String {
        let state = match self.is_valid() {
            true => "valid",
            false => "invalid",
        };
        format!("sha256:{} ({state})", self.value)
    }
}
