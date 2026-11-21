//! The complete experiment key and exact-build key.
//! Grounded in `RESEARCH_GATE.md:51-68` (the canonical JSON fields of `run_key`
//! and `exact_build_key`) and `:70-74` (the cache outcome vocabulary).

/// A canonical experiment key and its declared input fields.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Key {
    /// Digest of the canonical run key.
    pub run_digest: String,
    /// Digest of the exact build key.
    pub build_digest: String,
}

impl Key {
    /// The ordered fields hashed into the run key (`RESEARCH_GATE.md:55-63`).
    pub const RUN_INPUTS: [&'static str; 13] = [
        "binary_sha256",
        "runtime_policy_sha256",
        "command_spec_sha256",
        "fixture_sha256",
        "fixture_reset_digest",
        "oracle_version",
        "runner_image_digest",
        "cpu_topology_cohort",
        "effective_cgroup",
        "kernel_fs_cohort",
        "environment_digest",
        "measurement_protocol_sha256",
        "telemetry_mode",
    ];

    /// Assemble a key from already-computed trusted digests.
    #[must_use]
    pub fn of(run_digest: &str, build_digest: &str) -> Self {
        Self {
            run_digest: run_digest.to_owned(),
            build_digest: build_digest.to_owned(),
        }
    }

    /// Whether this key is complete enough for exact reuse.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        !self.run_digest.is_empty() && !self.build_digest.is_empty()
    }

    /// Whether two keys name the identical executable and environment.
    #[must_use]
    pub fn exactly_matches(&self, other: &Self) -> bool {
        self.run_digest == other.run_digest && self.build_digest == other.build_digest
    }
}
