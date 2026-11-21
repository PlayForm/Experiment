//! The eight-rung paper status ladder.
//!
//! Grounded in `schemas/paper.v5.schema.json:50-60` and
//! `PUBLISHING.md:13-28`: hypothesis_note, internal_observation,
//! measured_lab_report, replicated_report, sanitized_preprint,
//! externally_reviewed, corrected, retracted. The card names the same ladder
//! (hypothesis / internal note / verified lab report / independently
//! replicated / sanitized public preprint / externally peer reviewed) with
//! amendments. Nothing auto-promotes (`README.md:46-49`, `SYSTEM.md:118-122`).

use crate::Warning;

/// Explicit status of a versioned paper manuscript.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Status {
    /// Preregistered prediction with no empirical support (`PUBLISHING.md:15`).
    Hypothesis,
    /// An unreplicated observation or trace (`PUBLISHING.md:16`).
    Internal,
    /// Performance data attached with cohort and protocol (`PUBLISHING.md:19`).
    Measured,
    /// Independent confirmation across a separate run (`PUBLISHING.md:20-22`).
    Replicated,
    /// Confidentiality pass and signed release manifest (`PUBLISHING.md:23-25`).
    Sanitized,
    /// Genuine outside evaluation, never fabricated (`PUBLISHING.md:25-27`).
    Reviewed,
    /// Immutable amendment superseding prior bytes (`PUBLISHING.md:28`).
    Corrected,
    /// Immutable withdrawal, never destructive deletion (`PUBLISHING.md:28`).
    Retracted,
}

impl Status {
    /// All statuses in schema order.
    pub const ALL: [Self; 8] = [
        Self::Hypothesis,
        Self::Internal,
        Self::Measured,
        Self::Replicated,
        Self::Sanitized,
        Self::Reviewed,
        Self::Corrected,
        Self::Retracted,
    ];

    /// Decode the schema label, warning on an unknown one.
    pub fn from_label(label: &str) -> Result<Self, Warning> {
        match label {
            "hypothesis_note" => Ok(Self::Hypothesis),
            "internal_observation" => Ok(Self::Internal),
            "measured_lab_report" => Ok(Self::Measured),
            "replicated_report" => Ok(Self::Replicated),
            "sanitized_preprint" => Ok(Self::Sanitized),
            "externally_reviewed" => Ok(Self::Reviewed),
            "corrected" => Ok(Self::Corrected),
            "retracted" => Ok(Self::Retracted),
            other => Err(Warning::InvalidValue {
                field: "paper status",
                detail: format!("unknown status {other}"),
            }),
        }
    }

    /// The schema label of this status.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Hypothesis => "hypothesis_note",
            Self::Internal => "internal_observation",
            Self::Measured => "measured_lab_report",
            Self::Replicated => "replicated_report",
            Self::Sanitized => "sanitized_preprint",
            Self::Reviewed => "externally_reviewed",
            Self::Corrected => "corrected",
            Self::Retracted => "retracted",
        }
    }

    /// Public visibility begins only at the sanitized rung
    /// (`PUBLISHING.md:23-25`, `SYSTEM.md:106`).
    #[must_use]
    pub const fn requires_declassification(self) -> crate::Permission {
        match self {
            Self::Sanitized | Self::Reviewed => crate::Permission::Allow,
            Self::Hypothesis
            | Self::Internal
            | Self::Measured
            | Self::Replicated
            | Self::Corrected
            | Self::Retracted => crate::Permission::Deny,
        }
    }

    /// Amendments are terminal for their own bytes but remain searchable
    /// (`PUBLISHING.md:27-28`, `OPERATIONS.md:67`).
    #[must_use]
    pub const fn is_amendment(self) -> bool {
        matches!(self, Self::Corrected | Self::Retracted)
    }

    /// No rung is reached by prose alone (`README.md:46-49`).
    #[must_use]
    pub const fn may_auto_promote() -> crate::Permission {
        crate::Permission::Deny
    }
}
