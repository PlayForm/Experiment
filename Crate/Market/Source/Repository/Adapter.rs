//! The versioned repository-adapter manifest.
//!
//! Grounded in `repository-adapter.v5.schema.json:6-163` (17 required root
//! keys), `examples/repository-adapter.v5.json:2-29`,
//! `REPOSITORIES.md:3-40` (declarative manifest, capability manifest, source
//! scope, toolchain, commands, outputs, oracle, workloads, network and
//! publication policy) and `OPERATIONS.md:39-40` (scoped capabilities, never
//! arbitrary shell commands). This is validated data; no repository is touched.

use crate::{Digest, Warning};

use super::{Commands, Consent, License, Network, Outputs, Release, Toolchain, Visibility};

/// A versioned adapter contract for one source repository.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Adapter {
    /// Adapter identifier (`schema:29-33`).
    pub adapter_id: String,
    /// Repository identifier (`schema:34-38`).
    pub repo_id: String,
    /// Repository origin URI (`schema:39-42`).
    pub origin: String,
    /// Permitted base Git ref (`schema:43-47`).
    pub base_ref: String,
    /// Authorized owner identity (`schema:48-52`).
    pub authorized_owner: String,
    /// Repository visibility (`schema:53-55`).
    pub visibility: Visibility,
    /// License policy (`schema:56-58`).
    pub license_policy: License,
    /// Pinned toolchain (`schema:59-79`).
    pub toolchain: Toolchain,
    /// Authorized source paths (`schema:80-88`).
    pub source_scope: Vec<String>,
    /// Declared command vectors (`schema:89-116`).
    pub commands: Commands,
    /// Declared outputs (`schema:117-136`).
    pub outputs: Outputs,
    /// Correctness oracle identifier (`schema:137-141`).
    pub oracle_id: String,
    /// Deterministic workload identifiers (`schema:142-150`).
    pub workloads: Vec<String>,
    /// Outbound network policy (`schema:151-153`).
    pub default_network: Network,
    /// Release policy (`schema:154-156`).
    pub publication_policy: Release,
    /// Content address of the adapter manifest (`schema:157-160`).
    pub adapter_artifact_sha256: Digest,
    /// Owner consent record (`REPOSITORIES.md:13,78-83`).
    pub consent: Consent,
}

impl Adapter {
    /// Validate the structural contract of the manifest.
    pub fn validate(&self) -> Result<(), Warning> {
        self.toolchain.validate()?;
        self.commands.validate()?;
        self.outputs.validate()?;
        match (
            self.adapter_id.is_empty(),
            self.repo_id.is_empty(),
            self.base_ref.is_empty(),
            self.oracle_id.is_empty(),
            self.source_scope.is_empty(),
            self.workloads.is_empty(),
        ) {
            (false, false, false, false, false, false) => Ok(()),
            (true, _, _, _, _, _) => Err(Warning::MissingArtifact {
                digest: "adapter_id".to_owned(),
            }),
            (_, true, _, _, _, _) => Err(Warning::MissingArtifact {
                digest: "repo_id".to_owned(),
            }),
            (_, _, true, _, _, _) => Err(Warning::MissingArtifact {
                digest: "base_ref".to_owned(),
            }),
            (_, _, _, true, _, _) => Err(Warning::MissingArtifact {
                digest: "oracle_id".to_owned(),
            }),
            (_, _, _, _, true, _) => Err(Warning::MissingArtifact {
                digest: "source_scope".to_owned(),
            }),
            (_, _, _, _, _, true) => Err(Warning::MissingArtifact {
                digest: "workloads".to_owned(),
            }),
        }
    }

    /// A reliable oracle is required before a candidate may be called an
    /// improvement (`REPOSITORIES.md:82-83`).
    #[must_use]
    pub fn may_claim_improvement(&self) -> crate::Permission {
        self.consent.benchmark
    }

    /// An unknown adapter degrades to exploratory profiling
    /// (`OPERATIONS.md:38`, `REPOSITORIES.md:82-83`).
    #[must_use]
    pub fn consent_is_required(&self) -> crate::Permission {
        self.consent.is_complete()
    }
}
