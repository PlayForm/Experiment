//! Reporting cadence and required evidence from the playbook.
//!
//! Grounded in `playbooks/autonomous-week.yaml:92-102` and
//! `REQUIREMENTS.md:1034-1046` (reports must link machine-readable IDs and
//! include negative results). This type is the reporting contract as data;
//! no report file is generated.

use crate::Warning;

/// Snapshot cadence and required report contents.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reporting {
    pub snapshots: Vec<String>,
    pub include: Vec<String>,
}

impl Default for Reporting {
    fn default() -> Self {
        Self {
            snapshots: Self::SNAPSHOTS.iter().map(|item| (*item).to_owned()).collect(),
            include: Self::INCLUDE.iter().map(|item| (*item).to_owned()).collect(),
        }
    }
}

impl Reporting {
    /// `reporting.snapshots` (`autonomous-week.yaml:93`).
    pub const SNAPSHOTS: [&'static str; 3] = ["per_transition", "daily", "final"];

    /// `reporting.include` (`autonomous-week.yaml:95-102`).
    pub const INCLUDE: [&'static str; 6] = [
        "all_mutations",
        "negative_results",
        "raw_measurement_hashes",
        "lineage",
        "pareto_front",
        "unresolved_risks",
    ];

    /// Warn if a report would omit negative evidence or lineage
    /// (REQ-LAW-015, `REQUIREMENTS.md:1041-1042`).
    pub fn validate(&self, warnings: &mut Vec<Warning>) {
        Self::INCLUDE
            .into_iter()
            .filter(|required| !self.include.iter().any(|item| item == *required))
            .for_each(|missing| warnings.push(Warning::Unavailable {
                source: format!("reporting.include.{missing}"),
                substitute: "evidence omitted from weekly journal".to_owned(),
            }));
        match self.snapshots.is_empty() {
            true => warnings.push(Warning::Empty {
                domain: "reporting.snapshots",
            }),
            false => {}
        }
    }
}
