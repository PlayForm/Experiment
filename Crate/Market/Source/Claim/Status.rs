//! Evidence-status ladder attached to a claim link.
//!
//! Grounded in `paper.v5.schema.json:91-99`: unsupported,
//! external_reference, oracle_checked, measured, independently_replicated.
//! `SYSTEM.md:120-122` lists the separate signed decision states and insists
//! that each has explicit predecessor requirements and an immutable evidence
//! fingerprint. Status only rises when a named transition supplies the
//! predecessor; nothing here auto-promotes.

use crate::Warning;

/// The graded evidence status of one claim in a paper.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Status {
    /// No evidence attached (`paper schema:93`).
    Unsupported,
    /// A checked external reference (`paper schema:94`).
    ExternalReference,
    /// Semantic correctness under defined tests (`paper schema:95`).
    OracleChecked,
    /// Attached measurement with cohort and protocol (`paper schema:96`).
    Measured,
    /// Confirmed by an independent experiment (`paper schema:97`).
    IndependentlyReplicated,
}

impl Status {
    /// All evidence statuses in schema order.
    pub const ALL: [Self; 5] = [
        Self::Unsupported,
        Self::ExternalReference,
        Self::OracleChecked,
        Self::Measured,
        Self::IndependentlyReplicated,
    ];

    /// Decode the schema label, warning on an unknown one.
    pub fn from_label(label: &str) -> Result<Self, Warning> {
        match label {
            "unsupported" => Ok(Self::Unsupported),
            "external_reference" => Ok(Self::ExternalReference),
            "oracle_checked" => Ok(Self::OracleChecked),
            "measured" => Ok(Self::Measured),
            "independently_replicated" => Ok(Self::IndependentlyReplicated),
            other => Err(Warning::InvalidValue {
                field: "evidence status",
                detail: format!("unknown status {other}"),
            }),
        }
    }

    /// The wire label of this status.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
            Self::ExternalReference => "external_reference",
            Self::OracleChecked => "oracle_checked",
            Self::Measured => "measured",
            Self::IndependentlyReplicated => "independently_replicated",
        }
    }

    /// Only measured or independently replicated evidence is a measured fact
    /// (`README.md:41-43`, `PUBLISHING.md:74-77`).
    #[must_use]
    pub const fn is_measured_fact(self) -> crate::Permission {
        match self {
            Self::Measured | Self::IndependentlyReplicated => crate::Permission::Allow,
            Self::Unsupported | Self::ExternalReference | Self::OracleChecked => {
                crate::Permission::Deny
            }
        }
    }

    /// Ordinal rank used only to check predecessor requirements, never to
    /// auto-promote (`SYSTEM.md:124-126`).
    #[must_use]
    pub const fn rank(self) -> u8 {
        match self {
            Self::Unsupported => 0,
            Self::ExternalReference => 1,
            Self::OracleChecked => 2,
            Self::Measured => 3,
            Self::IndependentlyReplicated => 4,
        }
    }
}
