//! One panel in a plane-separated view.
//!
//! Grounded in `docs/12-dashboard.md:39-45` (UI is not the state machine or
//! trust authority; offline views show an explicit "stale since seq" marker)
//! and `docs/24-dual-observability-and-correspondence.md:1-11` (the dashboard
//! consumes event-ledger-backed projections only). A panel names the projection
//! it reads and the sequence it was materialized at; it never holds a write
//! path or a model-generated explanation.

/// A read-only panel rendered from a signed projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Panel {
    /// Stable panel identifier within its view.
    pub id: String,
    /// Ledger sequence the projection was materialized at.
    pub snapshot_seq: u64,
    /// Whether the panel may display decrypted private content.
    pub plaintext: crate::Permission,
    /// Whether the projection behind this panel is authoritative evidence.
    pub authoritative: crate::Permission,
}

impl Panel {
    /// Declare a panel; an empty identifier is refused with a warning.
    #[must_use]
    pub fn of(
        id: String,
        snapshot_seq: u64,
        plaintext: crate::Permission,
        authoritative: crate::Permission,
    ) -> Result<Self, crate::Warning> {
        match id.is_empty() {
            true => Err(crate::Warning::InvalidValue {
                field: "panel id",
                detail: "a panel must name its projection".to_owned(),
            }),
            false => Ok(Self {
                id,
                snapshot_seq,
                plaintext,
                authoritative,
            }),
        }
    }

    /// Whether this panel is stale relative to a newer ledger sequence.
    #[must_use]
    pub fn stale_against(&self, current: u64) -> crate::Permission {
        match self.snapshot_seq < current {
            true => crate::Permission::Allow,
            false => crate::Permission::Deny,
        }
    }
}
