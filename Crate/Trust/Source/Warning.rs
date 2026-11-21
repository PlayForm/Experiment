//! Recoverable warnings for the trust model's non-happy paths.
//!
//! A warning records a deviation that a caller can act on without aborting:
//! a missing certificate, a malformed config, an unreachable relay, a dead
//! helper subprocess, a colliding identity, an expired credential, a replay,
//! or an attempted mount of the offline root into a relay container. The
//! prototype's own guidance is to review, warn and degrade rather than crash
//! (`security/README.md:11-15`, `deploy/compose.dev.yaml:1-2`, defensive
//! non-happy-path rule). Nothing here panics.

use std::fmt;

/// A recoverable deviation from the trust happy path.
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
    /// A helper subprocess died or was never spawned.
    DeadSubprocess { helper: &'static str },
    /// A certificate, envelope or credential fell outside its validity window.
    Expired { subject: String },
    /// An identity appeared on the revocation list or was already revoked.
    Revoked { subject: String },
    /// A message ID was seen twice; replay must be refused, not retried.
    Replay { message_id: String },
    /// A config field could not be interpreted and a documented default applies.
    Malformed { domain: &'static str, detail: String },
    /// The offline CA directory was requested inside a relay or agent view.
    OfflineMountAttempt { view: &'static str },
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
                format!("warning: helper {helper} not available; optional path skipped")
            }
            Self::Expired { subject } => {
                format!("warning: credential for {subject} expired; access denied")
            }
            Self::Revoked { subject } => {
                format!("warning: identity {subject} revoked; access denied")
            }
            Self::Replay { message_id } => {
                format!("warning: message {message_id} already seen; replay refused")
            }
            Self::Malformed { domain, detail } => {
                format!("warning: malformed {domain} config ({detail}); using safe default")
            }
            Self::OfflineMountAttempt { view } => {
                format!("warning: refused offline CA mount into {view}; offline root stays isolated")
            }
        }
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.line())
    }
}
