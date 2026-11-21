//! Sanitization outcome of a submitted paper.
//!
//! Grounded in `paper.v5.schema.json:131-133` (`pending`, `passed`, `denied`),
//! `PUBLISHING.md:63-70` (sensitive scans over secrets, client paths,
//! proprietary source, licensing, personal data, code snippets, credentials,
//! agent instructions and prompt injection; uncertain cases go to human
//! review) and `SYSTEM.md:56` (the publisher validates policy-compliant
//! sanitization). This is a decision value; no document is scanned here.

use crate::Warning;

/// Outcome of the confidential sanitization pass.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Sanitization {
    /// Submitted but not yet reviewed (`paper schema:132`).
    Pending,
    /// Passed the release policy scan (`paper schema:132`).
    Passed,
    /// Denied; the paper stays internal (`paper schema:132`).
    Denied,
}

impl Sanitization {
    /// All outcomes in schema order.
    pub const ALL: [Self; 3] = [Self::Pending, Self::Denied, Self::Passed];

    /// The ten classes a sensitive scan must check (`PUBLISHING.md:63-64`).
    pub const SCAN_CLASSES: [&'static str; 10] = [
        "secrets",
        "client_paths",
        "proprietary_source",
        "repository_licensing",
        "personal_data",
        "code_snippets",
        "credentials",
        "agent_instructions",
        "prompt_injection",
        "unknown_external_repository",
    ];

    /// Decode the schema label, warning on an unknown one.
    pub fn from_label(label: &str) -> Result<Self, Warning> {
        match label {
            "pending" => Ok(Self::Pending),
            "passed" => Ok(Self::Passed),
            "denied" => Ok(Self::Denied),
            other => Err(Warning::InvalidValue {
                field: "sanitization status",
                detail: format!("unknown status {other}"),
            }),
        }
    }

    /// The wire label of this outcome.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Passed => "passed",
            Self::Denied => "denied",
        }
    }

    /// A release requires a passed pass plus a separate grant
    /// (`PUBLISHING.md:66-70`).
    #[must_use]
    pub const fn may_release(self) -> crate::Permission {
        match self {
            Self::Passed => crate::Permission::Allow,
            Self::Pending | Self::Denied => crate::Permission::Deny,
        }
    }
}
