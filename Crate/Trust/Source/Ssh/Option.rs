//! OpenSSH certificate critical options and extensions policy.
//!
//! Grounded in `security/src/ssh_ca.rs:57-59`: the prototype grants no
//! forwarding, no agent forwarding, and no unrestricted shell or PTY extensions,
//! and the real SSH server must separately enforce the principal and an
//! allow-list of commands. Each option is a typed capability, all denied by
//! default in this model. No SSH extension is granted here.
//!
//! Exception declared: this type is named `Option` because the card names the
//! file `Ssh/Option.rs`; std `Option` is not referenced in this module.

use crate::permission::Permission;

/// An OpenSSH certificate option or extension the prototype withholds.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Option {
    /// `permit-pty` extension (`ssh_ca.rs:57-59`).
    PermitPty,
    /// `permit-agent-forwarding` extension (`ssh_ca.rs:57-59`).
    PermitAgentForwarding,
    /// `permit-port-forwarding` extension (`ssh_ca.rs:57-59`).
    PermitPortForwarding,
    /// `permit-X11-forwarding` extension (`ssh_ca.rs:57-59`).
    PermitX11Forwarding,
    /// `permit-user-rc` extension (`ssh_ca.rs:57-59`).
    PermitUserRc,
    /// `force-command` critical option (`ssh_ca.rs:59`).
    ForceCommand,
    /// `source-address` critical option (`ssh_ca.rs:59`).
    SourceAddress,
}

impl Option {
    /// Every option the prototype leaves ungranted.
    pub const ALL: [Self; 7] = [
        Self::PermitPty,
        Self::PermitAgentForwarding,
        Self::PermitPortForwarding,
        Self::PermitX11Forwarding,
        Self::PermitUserRc,
        Self::ForceCommand,
        Self::SourceAddress,
    ];

    /// The canonical OpenSSH name of this option.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::PermitPty => "permit-pty",
            Self::PermitAgentForwarding => "permit-agent-forwarding",
            Self::PermitPortForwarding => "permit-port-forwarding",
            Self::PermitX11Forwarding => "permit-X11-forwarding",
            Self::PermitUserRc => "permit-user-rc",
            Self::ForceCommand => "force-command",
            Self::SourceAddress => "source-address",
        }
    }

    /// All options are denied by default (`ssh_ca.rs:57-59`).
    #[must_use]
    pub const fn is_granted(self) -> Permission {
        Permission::Deny
    }

    /// Whether a named option belongs to the withheld set.
    #[must_use]
    pub fn is_withheld(name: &str) -> bool {
        Self::ALL.iter().any(|option| option.name() == name)
    }
}
