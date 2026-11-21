//! A 64-hex SHA-256 identity, validated by shape only.
//!
//! A CAS digest is an identity, never a decryption entitlement and never a
//! worker-visible existence oracle (`docs/19:15-20`, `docs/25:12,16`).

/// A content address carried by reference; bytes are never hashed here.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Digest {
    value: String,
}

impl Digest {
    /// Wrap a digest string without computing a hash.
    #[must_use]
    pub fn of(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }

    /// An unset identity used only before a value is bound.
    #[must_use]
    pub fn zero() -> Self {
        Self::of("")
    }

    /// The wrapped string.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Whether the value is a 64-character lowercase-hex string.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.value.len() == crate::DIGEST_LENGTH
            && self.value.bytes().all(|byte| byte.is_ascii_hexdigit())
    }

    /// Whether two identities name the same bytes.
    #[must_use]
    pub fn matches(&self, other: &Self) -> crate::permission::Permission {
        match self == other {
            true => crate::permission::Permission::Allow,
            false => crate::permission::Permission::Deny,
        }
    }

    /// A description that never claims validity it has not checked.
    #[must_use]
    pub fn describe(&self) -> String {
        let state = match self.is_valid() {
            true => "valid",
            false => "invalid",
        };
        format!("sha256:{} ({state})", self.value)
    }
}
