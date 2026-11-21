//! Trusted cause codes for denied trade edges.
//!
//! The pinned example uses `anti-collusion`, `holdout`, `license`,
//! `cross-tenant` and `conflict-of-interest`. A cause is retained in audit and
//! is never an identifier or a message a worker can see
//! (`docs/20:40-42,57-70`).

/// One non-disclosing audit cause for a trade denial.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Reason {
    /// `anti-collusion` edge.
    Collusion,
    /// `holdout` separation.
    Holdout,
    /// License incompatibility across groups.
    License,
    /// `cross-tenant` boundary.
    Tenant,
    /// `conflict-of-interest` edge.
    Conflict,
    /// No trusted cause was available; fail closed.
    Unknown,
}

impl Reason {
    /// The exact wire spelling from the trade-restriction example.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Collusion => "anti-collusion",
            Self::Holdout => "holdout",
            Self::License => "license",
            Self::Tenant => "cross-tenant",
            Self::Conflict => "conflict-of-interest",
            Self::Unknown => "unknown",
        }
    }

    /// A trade cause is always audit-only.
    #[must_use]
    pub fn worker_visible(&self) -> bool {
        match self {
            Self::Collusion
            | Self::Holdout
            | Self::License
            | Self::Tenant
            | Self::Conflict
            | Self::Unknown => false,
        }
    }
}
