//! Trusted resolution of opaque schema handles.
//!
//! `docs/05:19` allows a standard `$ref` to target `urn:lab:schema:<id>` only
//! when the trusted resolver knows the mapping. `docs/10:2` rejects remote
//! `$ref` fetches and unknown namespaces. Two modes replace a boolean field:
//! a sealed private resolver and a published public view.

/// The mode in which a reference is resolved.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Resolver {
    /// Sealed registry resolver inside the authorization domain.
    Sealed,
    /// Public spec reader that must reject unknown or remote references.
    Published,
}

impl Resolver {
    /// Whether a supplied reference is acceptable in this mode.
    #[must_use]
    pub fn accepts(&self, reference: &str) -> crate::verdict::Verdict {
        match self {
            Self::Sealed => crate::verdict::Verdict::of(
                reference.starts_with(crate::SCHEMA_URN) && reference.len() > crate::SCHEMA_URN.len(),
            ),
            Self::Published => crate::verdict::Verdict::Deny,
        }
    }

    /// Remote resolution is always rejected, even for a sealed resolver
    /// (`docs/09:20-21`, `docs/10:2`).
    #[must_use]
    pub fn remote_fetch(reference: &str) -> crate::verdict::Verdict {
        crate::verdict::Verdict::of(!reference.contains("://"))
    }

    /// A resolved internal handle is not authority by itself; policy still
    /// evaluates identity, purpose and grant (`docs/05:34-37`).
    #[must_use]
    pub const fn confers_authority() -> bool {
        false
    }
}
