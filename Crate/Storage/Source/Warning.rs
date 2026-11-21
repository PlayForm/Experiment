//! Recoverable storage findings.
//!
//! A missing table, malformed digest, colliding identifier, stale epoch,
//! stale fence, destructive migration or diverging replay is a warning the
//! caller can act on. Nothing here aborts the process or fabricates a
//! successful apply (`docs/08-data-and-storage.md:78-80`,
//! `docs/14-runtime-and-recovery.md:24-40`).

use std::fmt;

/// One recoverable deviation from the storage happy path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Warning {
    /// A step would remove or alter existing schema; it is reported only.
    NonAdditive { version: String, change: String },
    /// A referenced table is not present in the current chain.
    MissingTable { name: String },
    /// Two definitions claimed the same namespace slot.
    Collision { domain: &'static str, id: String },
    /// A declared version differs from the version this model understands.
    Drift { domain: &'static str, expected: u32, found: u32 },
    /// A field could not be interpreted; a documented default applies.
    Malformed { domain: &'static str, detail: String },
    /// A digest is not a 64-character hex string.
    DigestShape { field: &'static str, detail: String },
    /// A lease, credential or fact fell outside its validity window.
    Expired { subject: String },
    /// A stale fencing token tried to commit authoritative output.
    Fenced { lease: String, expected: u64, found: u64 },
    /// A duplicate idempotency key was seen; replay is refused.
    Replay { key: String },
    /// Evidence was not verified and cannot be committed.
    Unverified { subject: String },
    /// Rebuilt projection state disagrees with the checkpoint.
    Divergence { expected: u64, observed: u64 },
    /// Partial or contaminated output was preserved but excluded.
    Quarantined { subject: String, reason: String },
    /// A private fact was withheld rather than disclosed to an untrusted view.
    Withheld { subject: &'static str },
    /// A trusted service was unreachable; the caller must degrade.
    Unreachable { service: &'static str },
    /// A referenced event is absent from the journal.
    MissingEvent { event_id: String },
    /// A cursor is older or newer than the sequence being read.
    OutOfOrder { have: u64, want: u64 },
    /// An aggregate sequence started below the required first value.
    InvalidSequence { expected: u64, found: u64 },
}

impl Warning {
    /// Render one human-readable diagnostic line.
    #[must_use]
    pub fn line(&self) -> String {
        match self {
            Self::NonAdditive { version, change } => {
                format!("warning: {version} step {change} is not additive; reported, not applied")
            }
            Self::MissingTable { name } => {
                format!("warning: table {name} missing; projection left rebuildable")
            }
            Self::Collision { domain, id } => {
                format!("warning: colliding {domain} identifier {id}")
            }
            Self::Drift { domain, expected, found } => {
                format!("warning: {domain} version drift, expected {expected}, found {found}")
            }
            Self::Malformed { domain, detail } => {
                format!("warning: malformed {domain} ({detail}); safe default retained")
            }
            Self::DigestShape { field, detail } => {
                format!("warning: digest field {field} ill-formed ({detail}); reference only")
            }
            Self::Expired { subject } => {
                format!("warning: {subject} expired; authority refused")
            }
            Self::Fenced { lease, expected, found } => {
                format!("warning: lease {lease} fenced, expected {expected}, found {found}")
            }
            Self::Replay { key } => {
                format!("warning: idempotency key {key} already recorded; replay refused")
            }
            Self::Unverified { subject } => {
                format!("warning: {subject} unverified; not committed")
            }
            Self::Divergence { expected, observed } => {
                format!("warning: replay divergence, expected seq {expected}, observed {observed}")
            }
            Self::Quarantined { subject, reason } => {
                format!("warning: {subject} quarantined ({reason}); preserved, excluded")
            }
            Self::Withheld { subject } => {
                format!("warning: {subject} withheld from untrusted view")
            }
            Self::Unreachable { service } => {
                format!("warning: {service} unreachable; ledger path continues")
            }
            Self::MissingEvent { event_id } => {
                format!("warning: event {event_id} missing; replay paused")
            }
            Self::OutOfOrder { have, want } => {
                format!("warning: sequence at {have}, wanted {want}; replay required")
            }
            Self::InvalidSequence { expected, found } => {
                format!("warning: aggregate sequence {found} invalid, expected at least {expected}")
            }
        }
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.line())
    }
}
