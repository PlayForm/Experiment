//! Signed, versioned, immutable repository adapter manifest.
//!
//! Grounded in `docs/13-repository-agnostic.md:4-11,17-30` and
//! `docs/26-acceptance-and-build-plan.md:8-10`: an adapter is a software module
//! with explicit entry points for dependency acquisition, source scoping,
//! deterministic build, trusted reference verification, dataset preparation
//! and benchmarking. It has no right to approve its own outputs and is executed
//! as untrusted or tightly scoped code until reviewed, because a novel adapter
//! can itself execute arbitrary build scripts (`docs/02:5-8`).

/// A validated adapter manifest; no build or command is executed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Adapter {
    /// Exact repository identity and pinned tree.
    pub repository: String,
    /// Toolchain and offline dependency closure.
    pub toolchain: String,
    /// Supported platform and architecture.
    pub platform: String,
    /// License policy applied to sources and dependencies.
    pub license: String,
    /// Whether the adapter may attest its own result.
    pub self_attestation: crate::Permission,
}

impl Adapter {
    /// Validate a manifest, refusing an empty repository identity.
    #[must_use]
    pub fn of(
        repository: String,
        toolchain: String,
        platform: String,
        license: String,
    ) -> Result<Self, crate::Warning> {
        match repository.is_empty() || toolchain.is_empty() {
            true => Err(crate::Warning::Missing {
                key: "adapter repository and toolchain",
            }),
            false => Ok(Self {
                repository,
                toolchain,
                platform,
                license,
                self_attestation: crate::Permission::Deny,
            }),
        }
    }

    /// The declared entry points an adapter may implement
    /// (`docs/13:4-9`).
    #[must_use]
    pub const fn entry_points() -> [&'static str; 6] {
        [
            "acquire-dependencies",
            "scope-source",
            "build-deterministic",
            "verify-reference",
            "prepare-dataset",
            "benchmark",
        ]
    }
}
