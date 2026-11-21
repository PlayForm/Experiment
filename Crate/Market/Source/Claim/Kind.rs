//! Claim kinds, separating opinion from measurement.
//!
//! Grounded in `schemas/claim.v5.schema.json:52-61` and
//! `sql/v5_migration.sql:65-73`: opinion, hypothesis, external_reference,
//! observation, interpretation and replication. `PUBLISHING.md:74-77` adds the
//! boundary that only observation and replication are measured facts; inference
//! and interpretation are model-derived or opinion. No kind becomes a fact by
//! being asserted persuasively.

use crate::Warning;

/// The declared kind of a signed claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Kind {
    /// A personal, falsifiable opinion (`schema:54`).
    Opinion,
    /// A preregistered mechanism hypothesis (`schema:55`).
    Hypothesis,
    /// An external literature or documentation reference (`schema:56`).
    ExternalReference,
    /// A trace or benchmark observation (`schema:57`).
    Observation,
    /// A model-derived interpretation (`schema:58`).
    Interpretation,
    /// A new independent experiment (`schema:59`).
    Replication,
}

impl Kind {
    /// All claim kinds in schema order.
    pub const ALL: [Self; 6] = [
        Self::Opinion,
        Self::Hypothesis,
        Self::ExternalReference,
        Self::Observation,
        Self::Interpretation,
        Self::Replication,
    ];

    /// Decode the schema label, warning on an unknown one.
    pub fn from_label(label: &str) -> Result<Self, Warning> {
        match label {
            "opinion" => Ok(Self::Opinion),
            "hypothesis" => Ok(Self::Hypothesis),
            "external_reference" => Ok(Self::ExternalReference),
            "observation" => Ok(Self::Observation),
            "interpretation" => Ok(Self::Interpretation),
            "replication" => Ok(Self::Replication),
            other => Err(Warning::InvalidValue {
                field: "claim type",
                detail: format!("unknown type {other}"),
            }),
        }
    }

    /// The wire label of this kind.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Opinion => "opinion",
            Self::Hypothesis => "hypothesis",
            Self::ExternalReference => "external_reference",
            Self::Observation => "observation",
            Self::Interpretation => "interpretation",
            Self::Replication => "replication",
        }
    }

    /// Whether this kind can carry a measured fact on its own
    /// (`README.md:41-43`, `PUBLISHING.md:74-77`). It cannot: every such claim
    /// still needs signed independent evidence.
    #[must_use]
    pub const fn is_fact_without_evidence() -> crate::Permission {
        crate::Permission::Deny
    }
}
