//! Reproducible build recipe and its exact build key.
//!
//! Grounded in `RESEARCH_GATE.md:64-68` (exact_build_key hashes source tree,
//! lockfile, toolchain digest, all Rust flags, Cargo features, target triple,
//! PGO/profile artifacts and reproducibility environment),
//! `ARCHITECTURE.md:76` (identical build keys reuse immutable artifacts) and
//! `CONTAINER_MATRIX.md:48-49` (pinned toolchain, network disabled once fetched,
//! no benchmark CPU entitlement). Same commit or feature set alone never proves
//! an identical binary.

/// The pinned inputs of one reproducible compile.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Recipe {
    /// Digest of the immutable source snapshot.
    pub source_sha256: String,
    /// Digest of the locked dependency set.
    pub locked_deps_sha256: String,
    /// Digest of the pinned toolchain.
    pub toolchain_sha256: String,
    /// Ordered Cargo features.
    pub features: Vec<String>,
    /// Ordered Rust flags.
    pub flags: Vec<String>,
    /// Target triple and CPU class.
    pub target: String,
    /// Cargo profile used for the compile.
    pub profile: String,
    /// Product instrumentation mode, part of build identity.
    pub instrumentation_mode: String,
    /// PGO profile digest, when this is a profile-guided build.
    pub pgo_profile_sha256: Option<String>,
    /// The resulting exact build key.
    pub build_key: String,
}

impl Recipe {
    /// Whether every required reproducibility input is pinned.
    #[must_use]
    pub fn is_pinned(&self) -> bool {
        [
            &self.source_sha256,
            &self.locked_deps_sha256,
            &self.toolchain_sha256,
            &self.target,
            &self.profile,
            &self.instrumentation_mode,
            &self.build_key,
        ]
        .iter()
        .all(|field| !field.is_empty())
    }

    /// The exact build key used for immutable artifact reuse.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.build_key
    }

    /// Whether a build is profile-guided and therefore a separate cohort.
    #[must_use]
    pub const fn is_profile_guided(&self) -> bool {
        self.pgo_profile_sha256.is_some()
    }
}
