//! The offline worker ABI: a typed contract, not an API grant.
//!
//! Grounded in `docs/11-agent-contracts.md:8-25`,
//! `docs/22-hidden-worker-and-correspondence.md:5-27` and
//! `docs/00-system-constitution.md:19-22`: the model receives read-only source,
//! task, references and optional private feedback, plus a writable scratch
//! directory; it has no network, no credentials and no service route. The role
//! prompt (proponent, skeptic, test author, drafter, integrator) cannot confer
//! authority (`docs/11:58-64`). This contract describes mounts and tools only.

/// The bounded input/output contract of one offline worker task.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Contract {
    /// Opaque task nonce; never a global campaign identifier.
    pub task_nonce: String,
    /// Digest of the pinned source snapshot made visible to the worker.
    pub source: crate::Digest,
    /// Network capability; must be denied.
    pub network: crate::Permission,
    /// Privileged socket capability; must be denied.
    pub privileged_socket: crate::Permission,
    /// Whether the worker holds any service credential.
    pub credentials: crate::Permission,
}

impl Contract {
    /// Build a sealed contract, refusing an empty task nonce with a warning.
    #[must_use]
    pub fn of(
        task_nonce: String,
        source: crate::Digest,
        network: crate::Permission,
        privileged_socket: crate::Permission,
        credentials: crate::Permission,
    ) -> Result<Self, crate::Warning> {
        match task_nonce.is_empty() {
            true => Err(crate::Warning::InvalidValue {
                field: "task nonce",
                detail: "an opaque task nonce is required".to_owned(),
            }),
            false => Ok(Self {
                task_nonce,
                source,
                network,
                privileged_socket,
                credentials,
            }),
        }
    }

    /// The only tools a sealed worker may invoke locally
    /// (`docs/22:13-21`).
    #[must_use]
    pub const fn tools() -> [&'static str; 5] {
        ["rg", "file-reader", "rustfmt", "cargo-check", "cargo-test"]
    }

    /// Whether this contract is safe to mount into an offline worker.
    #[must_use]
    pub const fn sealed(&self) -> crate::Permission {
        match (
            self.network,
            self.privileged_socket,
            self.credentials,
        ) {
            (
                crate::Permission::Deny,
                crate::Permission::Deny,
                crate::Permission::Deny,
            ) => crate::Permission::Allow,
            _ => crate::Permission::Deny,
        }
    }
}
