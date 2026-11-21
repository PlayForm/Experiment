//! Isolation of untrusted generated Rust build and test code.
//!
//! `README.md:34-36` and `docs/09:10-12` require generated build/test code to
//! be untrusted and isolated from trusted host processes, with a pinned
//! toolchain, offline dependencies and no secret mounts. `docs/10:39-51`
//! adds rootless isolation and denied host Docker socket. The worker model
//! gives deterministic software the authority over measurement and release.

use crate::verdict::Verdict;

/// One declared build-isolation precondition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Build {
    /// Toolchain and dependencies are pinned and offline.
    pub pinned_offline: Verdict,
    /// Generated code runs rootless with no host socket.
    pub rootless_isolated: Verdict,
    /// No secret or key material is mounted into the untrusted build.
    pub no_secret_mounts: Verdict,
    /// Evidence selection is performed by trusted deterministic software.
    pub trusted_selection: Verdict,
}

impl Build {
    /// A build is isolated only when every precondition allows.
    #[must_use]
    pub const fn isolated(&self) -> Verdict {
        self.pinned_offline
            .and(self.rootless_isolated)
            .and(self.no_secret_mounts)
            .and(self.trusted_selection)
    }

    /// A generated artifact never becomes authority over measurement or
    /// policy. Promotion and release are separate trusted decisions.
    #[must_use]
    pub const fn generated_is_authority() -> bool {
        false
    }
}
