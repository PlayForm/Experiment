//! Wide, server-filtered READ view granted to the director.
//!
//! Grounded in `ARCHITECTURE.md:40,123-125`, `AGENT_CONTRACTS.md:7`,
//! `REQUIREMENTS_V3.md` INV-002/DAT-002/CTX-007 and `STORAGE.md:4,6`. The view
//! is a disposable, authorized projection, never authoritative memory: access
//! is a broker/storage capability decision, not a prompt instruction.

use std::collections::BTreeMap;

use crate::permission::Permission;

/// A scoped read view over approved summaries and controlled deep inspection.
#[derive(Clone, Debug, Default)]
pub struct View {
    /// Approved campaign-wide summaries keyed by name.
    pub summaries: BTreeMap<String, String>,
    /// Whether controlled deep inspection is authorized for this session.
    pub deep_inspection: Permission,
    /// Redaction policy identifier applied to this view.
    pub redaction_policy_id: String,
}

impl View {
    /// Read an approved summary by key.
    #[must_use]
    pub fn summary(&self, key: &str) -> Option<&String> {
        self.summaries.get(key)
    }

    /// Whether a sealed-holdout record can be read through this view. It never
    /// can: only a narrow confirmer role obtains those records
    /// (`STORAGE.md:72-80`, `RESEARCH_GATE.md:78-81`).
    #[must_use]
    pub const fn may_read_holdout() -> Permission {
        Permission::Deny
    }

    /// The number of summaries in this view, for budget reporting.
    #[must_use]
    pub fn len(&self) -> usize {
        self.summaries.len()
    }

    /// Whether the view is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.summaries.is_empty()
    }
}
