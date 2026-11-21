//! The finite states recorded by v6 and v6.1 lease tables.

/// One lease state across task mailboxes, benchmark leases and storage leases.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
    /// Reserved but not yet staged.
    Reserved,
    /// Staged offline for a worker.
    Staged,
    /// Mounted into a sealed workspace.
    Mounted,
    /// Running under an exclusive holder.
    Running,
    /// Held by a benchmark host.
    Held,
    /// Active storage lease.
    Active,
    /// Sealed after authoritative completion.
    Sealed,
    /// Released by its holder.
    Released,
    /// Revoked by policy.
    Revoked,
    /// Past its expiry.
    Expired,
    /// Preserved but excluded after a fault.
    Quarantined,
}

impl State {
    /// Whether this state may produce authoritative output.
    #[must_use]
    pub fn authoritative(&self) -> crate::permission::Permission {
        match self {
            Self::Running | Self::Held | Self::Active | Self::Sealed => {
                crate::permission::Permission::Allow
            }
            Self::Reserved
            | Self::Staged
            | Self::Mounted
            | Self::Released
            | Self::Revoked
            | Self::Expired
            | Self::Quarantined => crate::permission::Permission::Deny,
        }
    }

    /// The declared state name.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Reserved => "reserved",
            Self::Staged => "staged",
            Self::Mounted => "mounted",
            Self::Running => "running",
            Self::Held => "held",
            Self::Active => "active",
            Self::Sealed => "sealed",
            Self::Released => "released",
            Self::Revoked => "revoked",
            Self::Expired => "expired",
            Self::Quarantined => "quarantined",
        }
    }
}
