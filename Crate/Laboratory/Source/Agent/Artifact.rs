//! A staged untrusted artifact, never an executable command.
//!
//! Grounded in `docs/00-system-constitution.md:23-25`,
//! `docs/17-v61-constitution.md:8-11` and `docs/15` BND-002: model output that
//! looks like an internal API request or a forged attestation is stored as
//! untrusted text and never executed. The supervisor validates path, type,
//! size, base source digest and scope before sealing a receipt
//! (`docs/22:23-27`). Knowledge of the artifact digest is not a grant
//! (`docs/08-data-and-storage.md:26-31`).

/// One staged output artifact from an offline worker.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Artifact {
    /// The declared proposal kind.
    pub proposal: crate::agent::Proposal,
    /// Scratch-relative path claimed by the worker.
    pub path: String,
    /// Digest of the staged bytes, once computed by the supervisor.
    pub digest: crate::Digest,
    /// Whether the trusted supervisor sealed a receipt.
    pub sealed: crate::Permission,
}

impl Artifact {
    /// Stage an artifact, refusing a path that could name a command or a
    /// privileged socket with a warning.
    #[must_use]
    pub fn of(
        proposal: crate::agent::Proposal,
        path: String,
        digest: crate::Digest,
        sealed: crate::Permission,
    ) -> Result<Self, crate::Warning> {
        let unsafe_path = path.contains("..") || path.starts_with("/");
        match unsafe_path {
            true => Err(crate::Warning::InvalidValue {
                field: "artifact path",
                detail: format!("scratch path escapes its root: {path}"),
            }),
            false => Ok(Self {
                proposal,
                path,
                digest,
                sealed,
            }),
        }
    }

    /// A staged artifact is data; execution authority is always denied.
    #[must_use]
    pub const fn executable(&self) -> crate::Permission {
        crate::Permission::Deny
    }

    /// Whether the artifact may feed a trusted ingestion gateway.
    #[must_use]
    pub const fn ingestible(&self) -> crate::Permission {
        self.sealed
    }
}
