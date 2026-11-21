//! One structural finding.
//!
//! A finding is a typed diagnostic. Missing references are non-fatal by
//! construction: `Finding::warning` is the default constructor used for every
//! unresolved schema or route (`docs/26-acceptance-and-build-plan.md:60-80`).

use crate::validate::Severity;

/// One validation finding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Finding {
    /// Finding severity.
    pub severity: Severity,
    /// Contract domain that produced the finding.
    pub domain: &'static str,
    /// Human-readable diagnostic message.
    pub message: String,
}

impl Finding {
    /// Build an informational finding.
    #[must_use]
    pub fn info(domain: &'static str, message: String) -> Self {
        Self { severity: Severity::Info, domain, message }
    }

    /// Build a recoverable warning; this is the missing-reference path.
    #[must_use]
    pub fn warning(domain: &'static str, message: String) -> Self {
        Self { severity: Severity::Warning, domain, message }
    }

    /// Build a contract-violation fault without aborting the run.
    #[must_use]
    pub fn fault(domain: &'static str, message: String) -> Self {
        Self { severity: Severity::Fault, domain, message }
    }

    /// Render one diagnostic line.
    #[must_use]
    pub fn line(&self) -> String {
        format!("{}: {}: {}", self.severity.as_str(), self.domain, self.message)
    }
}
