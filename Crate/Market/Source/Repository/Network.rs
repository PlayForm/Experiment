//! Network policy of an adapter.
//!
//! Grounded in `repository-adapter.v5.schema.json:151-153`
//! (`off`, `scoped_reference_only`), `SECURITY.md:69-74` (default no outbound
//! network for unknown code, egress only through a scoped reference proxy),
//! `REPOSITORIES.md:29-30` and `playbooks/research-market-week.yaml:71`
//! (`reference-broker-only`). No socket is opened here.

use crate::Warning;

/// Outbound network policy of an adapted repository.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Network {
    /// No outbound network (`schema:152`).
    Off,
    /// Egress only through a scoped reference broker (`schema:152`).
    ScopedReferenceOnly,
}

impl Network {
    /// Both network policies.
    pub const ALL: [Self; 2] = [Self::Off, Self::ScopedReferenceOnly];

    /// Decode the schema label, warning on an unknown one.
    pub fn from_label(label: &str) -> Result<Self, Warning> {
        match label {
            "off" => Ok(Self::Off),
            "scoped_reference_only" => Ok(Self::ScopedReferenceOnly),
            other => Err(Warning::InvalidValue {
                field: "network policy",
                detail: format!("unknown policy {other}"),
            }),
        }
    }

    /// The wire label of this policy.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::ScopedReferenceOnly => "scoped_reference_only",
        }
    }

    /// Arbitrary prompt-driven egress is never permitted
    /// (`OPERATIONS.md:39-40`, `SECURITY.md:72-74`).
    #[must_use]
    pub const fn may_egress_arbitrary_host() -> crate::Permission {
        crate::Permission::Deny
    }
}
