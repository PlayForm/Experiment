//! Recoverable warnings for the research economy's non-happy paths.
//!
//! A warning records a deviation a caller can act on without aborting: a
//! missing artifact, an unreachable broker, a dead helper, a duplicate bid, a
//! colliding case ID, a non-finite forecast, an exhausted cap, an expired
//! assignment, a replayed message, or an attempted promotion without its
//! predecessor evidence. The corpus asks for typed failure modes and bounded
//! refinement rather than crashes (`OPERATIONS.md:35-38`, `MARKET.md:70-79`,
//! `SYSTEM.md:118-122`). Nothing here panics.

use std::fmt;

/// A recoverable deviation from the research-economy happy path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Warning {
    /// A key was present in a document but is not part of the versioned model.
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
    /// A global, per-project, per-agent, per-model or per-topic cap was hit.
    CapExceeded { cap: &'static str, id: String },
    /// The requested native resource vector did not fit available capacity.
    Insufficient { resource: &'static str },
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
            Self::Insufficient { resource } => {
                format!("warning: insufficient {resource}; allocation refused")
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
