//! Owner consent to benchmark, modify and publish a repository.
//!
//! Grounded in `REPOSITORIES.md:13,78-83` (authorization to benchmark, modify
//! and publish; onboarding verifies owner consent before provisioning
//! secrets), `PUBLISHING.md:65-70` (no public release for unknown private or
//! third-party repositories) and `SYSTEM.md:74-78` (the owner controls
//! repository consent). Consent is deny-by-default typed data, not a grant.

use crate::Permission;

/// The three separate authorizations an adapter owner may give.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Consent {
    /// Owner identity that signed the consent (`REPOSITORIES.md:13`).
    pub owner_identity: String,
    /// Permission to execute trusted workloads.
    pub benchmark: Permission,
    /// Permission to submit atomic source patches.
    pub modify: Permission,
    /// Permission to publish a sanitized derivative.
    pub publish: Permission,
}

impl Consent {
    /// Construct consent with all three authorizations explicit.
    #[must_use]
    pub const fn of(
        owner_identity: String,
        benchmark: Permission,
        modify: Permission,
        publish: Permission,
    ) -> Self {
        Self {
            owner_identity,
            benchmark,
            modify,
            publish,
        }
    }

    /// Deny-by-default consent for an unknown owner.
    #[must_use]
    pub fn unknown(owner_identity: String) -> Self {
        Self::of(
            owner_identity,
            Permission::Deny,
            Permission::Deny,
            Permission::Deny,
        )
    }

    /// Whether all three authorizations were granted.
    #[must_use]
    pub fn is_complete(&self) -> Permission {
        self.benchmark.and(self.modify).and(self.publish)
    }

    /// An explicit benchmark consent does not imply publish consent
    /// (`REPOSITORIES.md:13`, `PUBLISHING.md:65-70`).
    #[must_use]
    pub const fn benchmark_implies_publish() -> Permission {
        Permission::Deny
    }
}
