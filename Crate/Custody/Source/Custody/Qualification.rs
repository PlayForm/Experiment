//! Holder roles from the custody qualification table.
//!
//! Mirrors the eight rows of `docs/19:53-64`. Each variant is a trusted role;
//! no role may hold a capability it is not attested for, and an offline
//! worker is never a signing, oracle or publishing authority
//! (`docs/17:82-96`).

use crate::permission::Permission;
use crate::right::Right;

/// A trusted holder role with a deliberately narrow capability set.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Qualification {
    /// Offline patcher (`docs/19:57`): source view, task, feedback, scratch.
    Patcher,
    /// Offline researcher (`docs/19:58`): extracts and own private context.
    Researcher,
    /// Builder (`docs/19:59`): build input and approved toolchain.
    Builder,
    /// Oracle (`docs/19:60`): fixtures, candidate binaries, controlled tests.
    Oracle,
    /// Runner (`docs/19:61`): frozen binaries and a hardware lease.
    Runner,
    /// Correspondence vault (`docs/19:62`): encrypted turns and transcripts.
    Vault,
    /// Publication custodian (`docs/19:63`): signed claims and review extracts.
    Custodian,
    /// Cold archive (`docs/19:64`): encrypted records under retention.
    Archive,
}

impl Qualification {
    /// Whether the role may hold a given right over an artifact.
    #[must_use]
    pub fn may_hold(self, right: Right) -> Permission {
        match (self, right) {
            (Self::Patcher | Self::Researcher | Self::Vault | Self::Archive, Right::Hold) => {
                Permission::Allow
            }
            (Self::Builder, Right::Hold | Right::Execute) => Permission::Allow,
            (Self::Oracle, Right::Hold | Right::Read) => Permission::Allow,
            (Self::Runner, Right::Hold | Right::Execute) => Permission::Allow,
            (Self::Custodian, Right::Hold | Right::Read) => Permission::Allow,
            _ => Permission::Deny,
        }
    }

    /// Whether this role is ever allowed to execute an offline model.
    #[must_use]
    pub fn is_worker(self) -> bool {
        match self {
            Self::Patcher | Self::Researcher => true,
            Self::Builder | Self::Oracle | Self::Runner | Self::Vault | Self::Custodian
            | Self::Archive => false,
        }
    }

    /// The wire spelling used by holder capability examples.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Patcher => "offline-patcher",
            Self::Researcher => "offline-researcher",
            Self::Builder => "builder",
            Self::Oracle => "oracle",
            Self::Runner => "runner",
            Self::Vault => "correspondence-vault",
            Self::Custodian => "publication-custodian",
            Self::Archive => "cold-archive",
        }
    }
}
