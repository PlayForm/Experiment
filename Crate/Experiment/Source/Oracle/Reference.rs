//! Frozen reference point for behavioral compatibility.
//!
//! Grounded in `ARCHITECTURE.md:82-86` (paired comparisons against frozen B0),
//! `REQUIREMENTS_V3.md` INV-004 (a candidate has a frozen absolute baseline
//! and a direct parent) and `playbooks/fanout-5.yaml:74-79` (paired baseline
//! frozen_B0, protocol locked). A reference cannot be silently replaced by a
//! candidate.

/// Whether a reference may be rewritten by a candidate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mutability {
    /// Pinned to the frozen trust root.
    Frozen,
    /// Replaceable only by a separately approved mutation.
    Mutable,
}

/// A frozen reference against which candidates are compared.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reference {
    /// Reference identifier, normally the frozen baseline B0.
    pub id: String,
    /// Digest of the frozen binary.
    pub binary_sha256: String,
    /// Digest of the fixture set used with this reference.
    pub fixture_sha256: String,
    /// Protocol version this reference is valid for.
    pub protocol_version: String,
    /// Whether this reference is pinned to the frozen trust root.
    pub mutability: Mutability,
}

impl Reference {
    /// Whether a candidate may redefine this reference. It never may
    /// (`REQUIREMENTS_V3.md` EXP-006).
    #[must_use]
    pub const fn may_be_redefined() -> bool {
        false
    }

    /// Whether this reference is comparable under a different protocol.
    #[must_use]
    pub fn matches_protocol(&self, protocol: &str) -> bool {
        self.protocol_version == protocol
    }

    /// The absolute baseline label this reference represents.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.id
    }
}
