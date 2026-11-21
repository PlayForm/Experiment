//! Build, oracle and benchmark command vectors.
//!
//! Grounded in `repository-adapter.v5.schema.json:89-116` (each of `build`,
//! `oracle`, `benchmark` is a non-empty string array) and `REPOSITORIES.md:23-25`
//! (correctness oracle with trusted fixtures, deterministic workload
//! definitions and sample protocol). Commands are inert data; nothing is
//! spawned and no shell is involved.

use crate::Warning;

/// The declared command vectors of an adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Commands {
    /// Hermetic build recipe (`schema:93-99`).
    pub build: Vec<String>,
    /// Correctness oracle invocation (`schema:100-105`).
    pub oracle: Vec<String>,
    /// Benchmark entrypoint (`schema:106-112`).
    pub benchmark: Vec<String>,
}

impl Commands {
    /// Validate that all three vectors are non-empty (`schema:91`).
    pub fn validate(&self) -> Result<(), Warning> {
        match (
            self.build.is_empty(),
            self.oracle.is_empty(),
            self.benchmark.is_empty(),
        ) {
            (false, false, false) => Ok(()),
            (true, _, _) => Err(Warning::MissingArtifact {
                digest: "commands.build".to_owned(),
            }),
            (_, true, _) => Err(Warning::MissingArtifact {
                digest: "commands.oracle".to_owned(),
            }),
            (_, _, true) => Err(Warning::MissingArtifact {
                digest: "commands.benchmark".to_owned(),
            }),
        }
    }

    /// A command vector is never a shell command from an arbitrary prompt
    /// (`OPERATIONS.md:39-40`, `SECURITY.md:68-74`).
    #[must_use]
    pub const fn is_untrusted_shell_input() -> crate::Permission {
        crate::Permission::Deny
    }
}
