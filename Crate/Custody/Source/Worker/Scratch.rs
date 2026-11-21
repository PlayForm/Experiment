//! Task-scoped writable scratch with canonical path containment.
//!
//! The supervisor canonicalizes submitted paths, rejects symlinks, hardlinks
//! and escapes, checks quota and seals an immutable receipt. A forbidden path
//! is treated exactly like an absent file, with no policy-specific message
//! (`docs/19:74`, `docs/22:19-21`, `docs/28:15-22`, `docs/25:19`).

use crate::network::Network;
use crate::permission::Permission;
use crate::switch::Switch;
use crate::warning::Warning;

/// The only directory an offline worker may write.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Scratch {
    /// Root path pinned to `/workspace/out`.
    pub root: String,
    /// Quota in bytes enforced by the supervisor.
    pub quota_bytes: u64,
    /// Bytes accepted so far.
    pub used_bytes: u64,
    /// Names accepted into this scratch directory.
    pub files: Vec<String>,
    /// Whether the output was sealed after ingestion.
    pub sealed: Switch,
    /// Network posture; must be `none` for a worker.
    pub network_mode: Network,
}

impl Scratch {
    /// Open a fresh scratch directory with the pinned root.
    #[must_use]
    pub fn of(quota: u64) -> Self {
        Self {
            root: crate::OUTPUT_DIRECTORY.to_owned(),
            quota_bytes: quota,
            used_bytes: 0,
            files: Vec::new(),
            sealed: Switch::Off,
            network_mode: Network::None,
        }
    }

    /// Whether a submitted path stays inside scratch without traversal.
    #[must_use]
    pub fn within(&self, path: &str) -> Permission {
        let rooted = path.starts_with(self.root.as_str());
        let traversal = path.split('/').any(|part| part == ".." || part == ".");
        let absolute_escape = path.contains('\0') || path.ends_with('/');
        Permission::of(rooted && !traversal && !absolute_escape)
    }

    /// Whether a bounded artifact may be accepted.
    #[must_use]
    pub fn accepts(&self, path: &str, bytes: u64) -> Permission {
        self.within(path)
            .and(Permission::of(bytes > 0))
            .and(Permission::of(
                self.used_bytes.saturating_add(bytes) <= self.quota_bytes,
            ))
            .and(self.network_mode.offline())
            .and(Permission::of(self.sealed == Switch::Off))
    }

    /// An escape finding for an operator; the worker sees only an absent file.
    #[must_use]
    pub fn finding(&self, path: &str) -> Option<Warning> {
        match self.within(path) {
            Permission::Allow => None,
            Permission::Deny => Some(Warning::Escape {
                path: path.to_owned(),
            }),
        }
    }

    /// Seal the directory so no further artifact can be accepted.
    #[must_use]
    pub fn seal(mut self) -> Self {
        self.sealed = Switch::On;
        self
    }
}
