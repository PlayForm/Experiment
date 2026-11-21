//! A typed candidate proposal.
//! Grounded in `ARCHITECTURE.md:73-76` (exactly one conceptual patch plus
//! content hashes) and `AGENT_CONTRACT.md:40-43` (structured final artifact
//! with mutation id, patch digest and modified paths).

use crate::propose::scope::Scope;

use super::origin::Origin;

/// An untrusted but typed candidate proposal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proposal {
    /// Opaque proposal identifier.
    pub id: String,
    /// Scope the proposal claims to mutate.
    pub scope: Scope,
    /// Digest of the normalized diff.
    pub digest: String,
    /// Provenance of the proposal.
    pub origin: Origin,
}

impl Proposal {
    /// Assemble a proposal from typed parts.
    #[must_use]
    pub fn of(id: &str, scope: Scope, digest: &str, origin: Origin) -> Self {
        Self {
            id: id.to_owned(),
            scope,
            digest: digest.to_owned(),
            origin,
        }
    }

    /// Whether the proposal is complete enough for the gate.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        !self.id.is_empty() && !self.digest.is_empty()
    }
}
