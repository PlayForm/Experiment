//! An accumulated validation report.
//!
//! A report is a typed collection of findings with typed summarizing methods.
//! It never panics on an empty or partially populated catalog: an empty
//! namespace becomes an informational finding rather than an assertion failure.

use crate::{validate::{Finding, Severity}, Permission};

/// A structural validation report.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Report {
    findings: Vec<Finding>,
}

impl Report {
    /// Build an empty report.
    #[must_use]
    pub const fn new() -> Self {
        Self { findings: Vec::new() }
    }

    /// Append one finding.
    #[must_use]
    pub fn with(mut self, finding: Finding) -> Self {
        self.findings.push(finding);
        self
    }

    /// Append many findings.
    #[must_use]
    pub fn extend(mut self, findings: Vec<Finding>) -> Self {
        self.findings.extend(findings);
        self
    }

    /// Merge another report.
    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        self.findings.extend(other.findings);
        self
    }

    /// Number of findings at a given severity.
    #[must_use]
    pub fn count(&self, severity: Severity) -> usize {
        self.findings.iter().filter(|finding| finding.severity == severity).count()
    }

    /// Total finding count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.findings.len()
    }

    /// Whether the report has no findings.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.findings.is_empty()
    }

    /// Whether the structural check passed: no fault findings.
    #[must_use]
    pub fn ok(&self) -> Permission {
        match self.count(Severity::Fault) {
            0 => Permission::Allow,
            _ => Permission::Deny,
        }
    }

    /// Render all findings as diagnostic lines.
    #[must_use]
    pub fn lines(&self) -> Vec<String> {
        self.findings.iter().map(Finding::line).collect()
    }
}
