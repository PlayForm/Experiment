//! The trust planes and projections an observer may be given.
//!
//! `docs/07:3-7` keeps Plane 1, Plane 2 and the private Plane -1 separate,
//! with Plane 0 as a read-only projection. `docs/07:28-30` says no LLM worker
//! sees any observability plane. Two structural properties replace booleans.

/// One observer view into the observability planes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum View {
    /// Plane 0 operator projection, redacted.
    Operator,
    /// Plane 1 laboratory event and queue metadata.
    PlaneOne,
    /// Plane 2 product profiling and benchmark metadata.
    PlaneTwo,
    /// Private Plane -1 mapping and key-policy audit.
    PrivateAudit,
    /// The offline LLM worker scratch view.
    Worker,
}

impl View {
    /// Whether this view can see a private mapping. Only the audited
    /// break-glass view can, and the access itself is logged (`docs/07:18-20`).
    #[must_use]
    pub const fn sees_private_mapping(&self) -> bool {
        matches!(self, Self::PrivateAudit)
    }

    /// Whether this view is the offline worker scratch view.
    #[must_use]
    pub const fn is_worker(&self) -> bool {
        matches!(self, Self::Worker)
    }

    /// No flow from any view into a worker observability context is allowed
    /// (`docs/07:28`).
    #[must_use]
    pub const fn worker_sees_observability() -> bool {
        false
    }

    /// Whether this view receives an operator-safe projection.
    #[must_use]
    pub const fn redacted(&self) -> bool {
        matches!(self, Self::Operator | Self::PlaneOne | Self::PlaneTwo | Self::Worker)
    }

    /// Stable label for an access decision.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Operator => "operator",
            Self::PlaneOne => "plane-1",
            Self::PlaneTwo => "plane-2",
            Self::PrivateAudit => "private-audit",
            Self::Worker => "worker",
        }
    }
}
