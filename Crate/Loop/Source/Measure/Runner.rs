//! The runner invocation model, bound to the SUT CLI surface.
//! Grounded in `ARCHITECTURE.md:56-58` (release benchmark container with signed
//! result manifests) and `AGENT_CONTRACT.md:47-61` (seeded paired order, keep
//! failures and contaminated trials). This value never spawns a process.

use runner::cli::command::Command;
use runner::{PACKAGE, VERSION};

use benchmark::runner::Group;

/// A typed description of one runner invocation against the SUT.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Runner {
    /// Path of the SUT CLI type consumed as a name.
    pub cli: String,
    /// SUT package name.
    pub package: String,
    /// SUT version under test.
    pub version: String,
    /// Isolated benchmark environment.
    pub group: Group,
}

impl Runner {
    /// Describe a runner invocation without executing it.
    #[must_use]
    pub fn of(_command: &Command, group: Group) -> Self {
        Self {
            cli: crate::RUNNER_CLI_PATH.to_owned(),
            package: PACKAGE.to_owned(),
            version: VERSION.to_owned(),
            group,
        }
    }

    /// Whether the invocation targets the pinned SUT.
    #[must_use]
    pub fn is_sut(&self) -> bool {
        self.package == PACKAGE && self.version == VERSION
    }

    /// Group tag used to name result artifacts.
    #[must_use]
    pub const fn tag(&self) -> &'static str {
        self.group.tag()
    }
}
