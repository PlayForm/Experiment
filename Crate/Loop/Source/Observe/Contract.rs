//! The reference behavioural contract of the SUT, read from the runner model.
//! Grounded in `00-system-constitution.md:5-9` (behavioural equivalence with
//! the approved reference contract is optimized first) and in
//! `26-acceptance-and-build-plan.md:74-79` (a worker's local `cargo test`
//! output never becomes authoritative correctness).

use runner::{PACKAGE, VERSION};

/// The typed identity of the behavioural contract under test.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Contract {
    /// SUT package name.
    pub package: String,
    /// SUT version under test.
    pub version: String,
    /// Trusted oracle version that certifies the contract.
    pub oracle_version: String,
}

impl Contract {
    /// Build the contract from the pinned SUT identity.
    #[must_use]
    pub fn of(oracle_version: &str) -> Self {
        Self {
            package: PACKAGE.to_owned(),
            version: VERSION.to_owned(),
            oracle_version: oracle_version.to_owned(),
        }
    }

    /// Whether the contract is pinned to a trusted oracle version.
    #[must_use]
    pub fn is_pinned(&self) -> bool {
        !self.oracle_version.is_empty()
    }

    /// Whether another contract names the same SUT identity.
    #[must_use]
    pub fn same_sut(&self, other: &Self) -> bool {
        self.package == other.package && self.version == other.version
    }
}
