//! The result of reviewing one migration step.

use super::version::Version;
use crate::permission::Permission;

/// A reviewed step is accepted as additive or reported for human review.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Finding {
    /// Classified additive and eligible for a trusted applier; not applied here.
    Accept(Version),
    /// A destructive change was found; it is reported, never performed.
    Reported(Version, crate::warning::Warning),
}

impl Finding {
    /// The version this finding concerns.
    #[must_use]
    pub fn version(&self) -> Version {
        match self {
            Self::Accept(version) | Self::Reported(version, _) => *version,
        }
    }

    /// Whether the step may proceed.
    #[must_use]
    pub fn verdict(&self) -> Permission {
        match self {
            Self::Accept(_) => Permission::Allow,
            Self::Reported(_, _) => Permission::Deny,
        }
    }

    /// Render the finding as one diagnostic line.
    #[must_use]
    pub fn line(&self) -> String {
        match self {
            Self::Accept(version) => {
                format!("{}: additive chain step classified; no SQL executed", version.label())
            }
            Self::Reported(version, warning) => {
                format!("{}: {}", version.label(), warning.line())
            }
        }
    }
}
