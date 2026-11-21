//! The offline worker actor and its file-only interface.
//!
//! Grounded in `docs/11-agent-contracts.md:8-25`,
//! `docs/22-hidden-worker-and-correspondence.md:5-27`,
//! `docs/15-acceptance-tests.md` BND-001..007 and `docs/00:19-22`: the worker
//! has no network, no Docker socket, no internal/external API route, no keys,
//! no sibling files and no telemetry SDK. It receives read-only mounts and
//! returns staged untrusted files through a scratch directory. Every denial is
//! a typed `Permission::Deny`; the worker is never told which rule fired
//! (`docs/22:46-69`).

/// The capability envelope of an offline worker.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Worker {
    /// Network connectivity of the model process.
    pub network: crate::Permission,
    /// Reachability of the public API from the worker.
    pub public_api: crate::Permission,
    /// Reachability of the internal API from the worker.
    pub internal_api: crate::Permission,
    /// Access to the host Docker socket or privileged mounts.
    pub docker: crate::Permission,
    /// Possession of any service credential or key.
    pub credentials: crate::Permission,
    /// Visibility of sibling correspondence.
    pub sibling_files: crate::Permission,
}

impl Worker {
    /// The only valid worker envelope: every authority denied.
    #[must_use]
    pub const fn offline() -> Self {
        Self {
            network: crate::Permission::Deny,
            public_api: crate::Permission::Deny,
            internal_api: crate::Permission::Deny,
            docker: crate::Permission::Deny,
            credentials: crate::Permission::Deny,
            sibling_files: crate::Permission::Deny,
        }
    }

    /// Whether every authority is denied, so the worker is sealed.
    #[must_use]
    pub const fn is_sealed(&self) -> crate::Permission {
        match (
            self.network,
            self.public_api,
            self.internal_api,
            self.docker,
            self.credentials,
            self.sibling_files,
        ) {
            (
                crate::Permission::Deny,
                crate::Permission::Deny,
                crate::Permission::Deny,
                crate::Permission::Deny,
                crate::Permission::Deny,
                crate::Permission::Deny,
            ) => crate::Permission::Allow,
            _ => crate::Permission::Deny,
        }
    }

    /// The read-only and writable mounts a sealed worker may see
    /// (`docs/22:5-8`).
    #[must_use]
    pub const fn mounts() -> [&'static str; 5] {
        [
            "/workspace/repo",
            "/workspace/task.txt",
            "/workspace/references",
            "/workspace/feedback.txt",
            "/workspace/out",
        ]
    }
}
