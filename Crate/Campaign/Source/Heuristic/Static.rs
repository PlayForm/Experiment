//! Static heuristic mode: parsed options and cheap facts only.
//!
//! Grounded in `REQUIREMENTS.md:200-201` and REQ-HEU-04..08
//! (`:223-228`): no program-name inference, no speculative execution, no
//! variable expansion, no shell globbing, no user-command probing. Metadata
//! sampling is bounded by depth, item count, bytes, syscalls and elapsed time.
//! These are typed capabilities; no analysis is run here.

use crate::agent::Permission;

/// The bounds and prohibitions of the static mode.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Static {
    pub max_features: u32,
    pub max_depth: u32,
    pub max_items: u32,
    pub max_bytes: u64,
    pub max_syscalls: u32,
    pub max_elapsed_us: u64,
    pub expand_variables: Permission,
    pub shell_globbing: Permission,
    pub command_probing: Permission,
}

impl Default for Static {
    fn default() -> Self {
        Self {
            max_features: 64,
            max_depth: 2,
            max_items: 1_024,
            max_bytes: 65_536,
            max_syscalls: 256,
            max_elapsed_us: 200,
            expand_variables: Permission::Deny,
            shell_globbing: Permission::Deny,
            command_probing: Permission::Deny,
        }
    }
}

impl Static {
    /// The five REQ-HEU-08 caps in declaration order.
    #[must_use]
    pub const fn caps(&self) -> (u32, u32, u64, u32, u64) {
        (
            self.max_depth,
            self.max_items,
            self.max_bytes,
            self.max_syscalls,
            self.max_elapsed_us,
        )
    }

    /// Whether the mode honours the zero-hidden-execution law (LAW-003).
    #[must_use]
    pub fn is_passive(&self) -> bool {
        self.expand_variables == Permission::Deny
            && self.shell_globbing == Permission::Deny
            && self.command_probing == Permission::Deny
    }
}
