//! The eight-state research-case lifecycle.
//!
//! Grounded in `schemas/research-case.v5.schema.json:62-73` and the longer
//! `SYSTEM.md:106` lifecycle (`opened -> blind_assigned -> preregistered ->
//! ... -> sealed_internal|released_public -> available_for_transfer`). Both
//! encodings are named states; a transition is never inferred from prose.

use crate::Warning;

/// Lifecycle state of a pre-registered research case.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum State {
    /// Case opened with a frozen question and scope (`schema:64`).
    Opened,
    /// Blind researchers assigned from distinct capsules (`schema:65`).
    BlindAssigned,
    /// Claims and predictions frozen before outcomes (`schema:66`).
    Preregistered,
    /// Measurements attached with cohort and protocol (`schema:67`).
    Measured,
    /// Independent evaluator produced a signed verdict (`schema:68`).
    Evaluated,
    /// Peer critique and replication reviewed (`schema:69`).
    Reviewed,
    /// Publication bridge sealed or released the case (`schema:70`).
    Published,
    /// Case closed, still searchable (`schema:71`, `SYSTEM.md:126`).
    Closed,
}

impl State {
    /// All lifecycle states in schema order.
    pub const ALL: [Self; 8] = [
        Self::Opened,
        Self::BlindAssigned,
        Self::Preregistered,
        Self::Measured,
        Self::Evaluated,
        Self::Reviewed,
        Self::Published,
        Self::Closed,
    ];

    /// Decode the schema label, warning on an unknown one.
    pub fn from_label(label: &str) -> Result<Self, Warning> {
        match label {
            "opened" => Ok(Self::Opened),
            "blind_assigned" => Ok(Self::BlindAssigned),
            "preregistered" => Ok(Self::Preregistered),
            "measured" => Ok(Self::Measured),
            "evaluated" => Ok(Self::Evaluated),
            "reviewed" => Ok(Self::Reviewed),
            "published" => Ok(Self::Published),
            "closed" => Ok(Self::Closed),
            other => Err(Warning::InvalidValue {
                field: "case state",
                detail: format!("unknown label {other}"),
            }),
        }
    }

    /// The schema label of this state.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Opened => "opened",
            Self::BlindAssigned => "blind_assigned",
            Self::Preregistered => "preregistered",
            Self::Measured => "measured",
            Self::Evaluated => "evaluated",
            Self::Reviewed => "reviewed",
            Self::Published => "published",
            Self::Closed => "closed",
        }
    }

    /// A closed case remains searchable; nothing is destructively deleted
    /// (`SYSTEM.md:126`, `PUBLISHING.md:27-28`).
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Closed)
    }
}
