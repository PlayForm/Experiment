//! Recoverable warnings for the laboratory's non-happy paths.
//!
//! A warning records a deviation a caller can act on without aborting: a
//! missing artifact, an unreachable service, a dead subprocess, a stale policy
//! epoch, a fenced lease, a contaminated benchmark window, an inconclusive
//! oracle, a replayed command or a withheld disclosure. The corpus requires
//! typed failure and inconclusive states instead of crashes
//! (`docs/06-state-machines.md:92-98`, `docs/25-failure-analysis.md:36-58`,
//! `docs/17-v61-constitution.md:65-79`). Nothing here panics or fabricates a
//! passing result.

use std::fmt;

/// A recoverable deviation from the laboratory happy path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Warning {
    /// A key was present but is not part of the versioned model.
    Unknown { domain: &'static str, key: String },
    /// A declared version differs from the version this model understands.
    Drift { domain: &'static str, expected: u32, found: u32 },
    /// Two identifiers claimed the same namespace slot.
    Collision { domain: &'static str, id: String },
    /// A referenced artifact was absent; the caller must degrade.
    MissingArtifact { digest: String },
    /// A named service was unreachable; the caller must degrade, not panic.
    Unreachable { service: &'static str },
    /// A helper subprocess died or could not be spawned.
    DeadSubprocess { helper: &'static str },
    /// A field could not be interpreted and a documented default applies.
    Malformed { domain: &'static str, detail: String },
    /// A credential, reservation or assignment fell outside its window.
    Expired { subject: String },
    /// A transaction or message ID was seen twice; replay is refused.
    Replay { message_id: String },
    /// An exact-duplicate identifier or payload was rejected.
    Duplicate { domain: &'static str, id: String },
    /// A numeric field was non-finite or outside its declared range.
    InvalidValue { field: &'static str, detail: String },
    /// A cap, quota or physical capacity was exhausted.
    CapExceeded { cap: &'static str, id: String },
    /// A lease fence rejected a stale token.
    Fenced { lease: String },
    /// A policy epoch was stale and the operation was re-evaluated.
    StaleEpoch { expected: u64, found: u64 },
    /// Evidence was contaminated by a host or topology drift event.
    Contaminated { reason: String },
    /// A result was inconclusive; uncertainty must be preserved.
    Inconclusive { subject: String },
    /// A private fact was withheld rather than disclosed to an untrusted view.
    Withheld { subject: String },
    /// A projection was read at a sequence older than the requested one.
    StaleProjection { have: u64, want: u64 },
    /// An action was refused by a signed policy decision.
    Denied { subject: &'static str, reason: &'static str },
}

impl Warning {
    /// Render one human-readable diagnostic line.
    #[must_use]
    pub fn line(&self) -> String {
        match self {
            Self::Unknown { domain, key } => {
                format!("warning: unknown {domain} key {key} ignored")
            }
            Self::Drift { domain, expected, found } => {
                format!("warning: {domain} version drift, expected {expected}, found {found}")
            }
            Self::Collision { domain, id } => {
                format!("warning: colliding {domain} identifier {id}")
            }
            Self::MissingArtifact { digest } => {
                format!("warning: artifact {digest} missing; reference only")
            }
            Self::Unreachable { service } => {
                format!("warning: service {service} unreachable; degrading")
            }
            Self::DeadSubprocess { helper } => {
                format!("warning: helper {helper} unavailable; optional path skipped")
            }
            Self::Malformed { domain, detail } => {
                format!("warning: malformed {domain} input ({detail}); using safe default")
            }
            Self::Expired { subject } => {
                format!("warning: {subject} expired; access denied")
            }
            Self::Replay { message_id } => {
                format!("warning: {message_id} already seen; replay refused")
            }
            Self::Duplicate { domain, id } => {
                format!("warning: duplicate {domain} {id}; exact-duplicate gate closed")
            }
            Self::InvalidValue { field, detail } => {
                format!("warning: invalid {field} ({detail}); value rejected")
            }
            Self::CapExceeded { cap, id } => {
                format!("warning: cap {cap} reached by {id}; allocation refused")
            }
            Self::Fenced { lease } => {
                format!("warning: lease {lease} fenced; stale authority rejected")
            }
            Self::StaleEpoch { expected, found } => {
                format!("warning: policy epoch stale, expected {expected}, found {found}; recheck")
            }
            Self::Contaminated { reason } => {
                format!("warning: sample contaminated ({reason}); excluded, not corrected")
            }
            Self::Inconclusive { subject } => {
                format!("warning: {subject} inconclusive; uncertainty preserved")
            }
            Self::Withheld { subject } => {
                format!("warning: {subject} withheld from untrusted view")
            }
            Self::StaleProjection { have, want } => {
                format!("warning: projection at seq {have}, wanted {want}; stale until replay")
            }
            Self::Denied { subject, reason } => {
                format!("warning: {subject} denied: {reason}")
            }
        }
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.line())
    }
}
