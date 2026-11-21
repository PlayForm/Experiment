//! Recoverable warnings for a degraded, non-happy-path laboratory.
//!
//! A warning records a deviation that policy can act on without aborting the
//! run: an unknown schema key, a version drift, a colliding identifier, a
//! missing artifact, an unreachable service, a dead lease, or an exporter that
//! dropped spans. The event ledger remains authoritative even when telemetry is
//! absent (`TELEMETRY.md:6`, `TELEMETRY.md:158`, `REQUIREMENTS_V3.md` TEL-007).

use std::fmt;

/// A recoverable deviation from the happy path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Warning {
    /// A key was present in a document but is not part of the versioned schema.
    Unknown { domain: &'static str, key: String },
    /// A declared version differs from the version this model understands.
    Drift { domain: &'static str, expected: u32, found: u32 },
    /// Two identifiers collided within one namespace.
    Collision { domain: &'static str, id: String },
    /// A referenced artifact was missing; the caller must degrade.
    MissingArtifact { digest: String },
    /// A service was unreachable; the caller must degrade, not panic.
    Unreachable { service: &'static str },
    /// A lease or heartbeat expired and its work was reclaimed.
    ExpiredLease { work_order: String },
    /// A telemetry exporter dropped spans; raw timing facts still stand.
    DroppedSpans { plane: &'static str, count: u64 },
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
            Self::ExpiredLease { work_order } => {
                format!("warning: lease for {work_order} expired; reclaimed")
            }
            Self::DroppedSpans { plane, count } => {
                format!("warning: {plane} plane dropped {count} spans; evidence retained")
            }
        }
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.line())
    }
}
