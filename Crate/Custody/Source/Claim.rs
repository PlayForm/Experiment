//! Worker-supplied success claim, explicitly not authoritative.
//!
//! `claim_of_success` is limited to `not-claimed`, `local-test-only` and
//! `unknown`, and no value may be read as acceptance, benchmark completion or
//! publication (`docs/28:24-27`; offline submission schema).

/// The only self-assessment a submission may carry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Claim {
    /// The worker makes no success claim (`not-claimed`).
    None,
    /// Only local untrusted test output was observed (`local-test-only`).
    Local,
    /// The worker states no assessment (`unknown`).
    Unknown,
}

impl Claim {
    /// The wire spelling from the offline submission schema.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::None => "not-claimed",
            Self::Local => "local-test-only",
            Self::Unknown => "unknown",
        }
    }

    /// No worker claim can carry release authority.
    #[must_use]
    pub fn authoritative(&self) -> bool {
        match self {
            Self::None | Self::Local | Self::Unknown => false,
        }
    }
}
